use common::{Itertools, SS, second, swap, to_usize};
use id_arena::{Arena, Id};
use std::{
    collections::{BTreeSet, HashMap},
    fmt::{Display, Write},
};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum WireKind {
    X(usize),
    Y(usize),
    Z(usize),
    Other(SS),
}

impl Display for WireKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // silly, I know
        match self {
            WireKind::X(nr) => {
                f.write_char('x')?;
                nr.fmt(f)
            }
            WireKind::Y(nr) => {
                f.write_char('y')?;
                nr.fmt(f)
            }
            WireKind::Z(nr) => {
                f.write_char('z')?;
                nr.fmt(f)
            }
            WireKind::Other(s) => f.write_str(s),
        }
    }
}

impl WireKind {
    pub fn as_z(&self) -> Option<usize> {
        if let Self::Z(v) = self {
            Some(*v)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub struct Wire {
    pub id: Id<Wire>,
    pub name: WireKind,
    pub input: Option<Id<Gate>>,
    pub outputs: BTreeSet<Id<Gate>>,
    pub value: Option<usize>,
}

impl Wire {
    pub fn new(id: Id<Wire>, name: SS) -> Self {
        let name = match name.split_at(1) {
            ("x", nr) => WireKind::X(to_usize(nr)),
            ("y", nr) => WireKind::Y(to_usize(nr)),
            ("z", nr) => WireKind::Z(to_usize(nr)),
            _ => WireKind::Other(name),
        };
        Self {
            id,
            name,
            input: None,
            outputs: Default::default(),
            value: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Gate {
    pub inp: [Id<Wire>; 2],
    pub op: Op,
    pub out: Id<Wire>,
}

#[derive(Default)]
pub struct Circuit {
    gates: Arena<Gate>,
    wires: Arena<Wire>,
    wire_map: HashMap<SS, Id<Wire>>,
}

impl Circuit {
    pub fn wire_id(&mut self, name: SS) -> Id<Wire> {
        *self
            .wire_map
            .entry(name)
            .or_insert_with(|| self.wires.alloc_with_id(|id| Wire::new(id, name)))
    }

    pub fn wire(&self, id: Id<Wire>) -> &Wire {
        &self.wires[id]
    }

    pub fn expect_same_wires(&self, a: Id<Wire>, b: Id<Wire>, ctx: impl Display) {
        if a == b {
            return;
        }
        let a = self.wire(a).name;
        let b = self.wire(b).name;
        panic!("wire mismatch in {ctx}, got: {a} and {b}");
    }

    pub fn expect_input_to_contain(
        &self,
        gate: &Gate,
        a: Id<Wire>,
        b: Id<Wire>,
        expected: impl Display,
    ) {
        if gate.inp.contains(&a) && gate.inp.contains(&b) {
            return;
        }
        panic!(
            "expect {expected}, gate inputs: [{}], wire a: {}, wire b: {}",
            gate.inp.iter().map(|&id| self.wire(id).name).join(","),
            self.wire(a).name,
            self.wire(b).name,
        );
    }

    pub fn expect_same_outputs(&self, a: Id<Wire>, b: Id<Wire>, ctx: impl Display) {
        let a = &self.wire(a);
        let b = &self.wire(b);
        if a.outputs == b.outputs {
            return;
        }
        let expected_from = |wire: &Wire| {
            wire.outputs
                .iter()
                .map(|&g| {
                    let id = *self
                        .gate(g)
                        .inp
                        .iter()
                        .filter(|&&i| i != wire.id)
                        .exactly_one()
                        .unwrap();
                    self.wire(id).name
                })
                .collect_vec()
        };

        panic!(
            "wires {} and {} are not connected to the same gates in {ctx}, \
             expected from left: {:?}, expected from right: {:?}",
            a.name,
            b.name,
            expected_from(a),
            expected_from(b)
        )
    }

    pub fn gate(&self, id: Id<Gate>) -> &Gate {
        &self.gates[id]
    }

    pub fn output_gates<'a>(
        &'a self,
        ids: impl IntoIterator<Item = &'a Id<Gate>>,
    ) -> impl Iterator<Item = &'a Gate> {
        ids.into_iter().copied().map(|id| &self.gates[id])
    }

    pub fn wires(&self) -> impl Iterator<Item = &Wire> {
        self.wires.iter().map(second)
    }

    pub fn eval(&mut self, id: Id<Wire>) -> usize {
        let wire = &self.wires[id];
        if let Some(value) = wire.value {
            return value;
        }
        let gate_id = wire.input.expect("wire should have a value or an input");
        let gate = &self.gates[gate_id];
        let [a, b] = gate.inp;
        let value = match gate.op {
            Op::And => self.eval(a) & self.eval(b),
            Op::Or => self.eval(a) | self.eval(b),
            Op::Xor => self.eval(a) ^ self.eval(b),
        };
        self.wires[id].value = Some(value);
        value
    }

    pub fn parse(input: SS, swaps: &[(SS, SS)]) -> Self {
        let swaps: HashMap<_, _> = swaps
            .iter()
            .copied()
            .chain(swaps.iter().copied().map(swap))
            .collect();
        let mut circuit = Self::default();
        let mut lines = input.lines();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let (name, value) = line.split_once(": ").unwrap();
            let id = circuit.wire_id(name);
            circuit.wires[id].value = Some(to_usize(value));
        }

        for line in lines {
            let (lhs, op, rhs, arrow, out) = line.split_whitespace().collect_tuple().unwrap();
            assert_eq!(arrow, "->");
            let op = match op {
                "AND" => Op::And,
                "OR" => Op::Or,
                "XOR" => Op::Xor,
                _ => panic!("unknown gate: {op}"),
            };
            let mut inp = [circuit.wire_id(lhs), circuit.wire_id(rhs)];
            inp.sort();
            let out = swaps.get(out).copied().unwrap_or(out);
            let out = circuit.wire_id(out);
            let id = circuit.gates.alloc(Gate { inp, op, out });
            circuit.wires[inp[0]].outputs.insert(id);
            circuit.wires[inp[1]].outputs.insert(id);
            circuit.wires[out].input = Some(id);
        }
        circuit
    }
}
