use common::{Itertools, SS, second, swap, to_usize};
use id_arena::{Arena, Id};
use std::{
    collections::{BTreeSet, HashMap},
    fmt::Display,
};

#[derive(Clone, Debug)]
pub struct Wire {
    pub name: SS,
    pub input: Option<Id<Gate>>,
    pub outputs: BTreeSet<Id<Gate>>,
    pub value: Option<usize>,
}

impl Wire {
    pub fn new(name: SS) -> Self {
        Self {
            name,
            input: None,
            outputs: Default::default(),
            value: None,
        }
    }

    pub fn as_io(&self) -> Option<(char, usize)> {
        match self.name.split_at(1) {
            (kind @ ("x" | "y" | "z"), nr) => {
                Some((kind.chars().exactly_one().unwrap(), to_usize(nr)))
            }
            _ => None,
        }
    }

    pub fn as_z(&self) -> Option<usize> {
        self.as_io().filter(|(kind, _)| *kind == 'z').map(second)
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
    pub inp: [SS; 2],
    pub op: Op,
    pub out: SS,
}

#[derive(Default)]
pub struct Circuit {
    gates: Arena<Gate>,
    wires: HashMap<SS, Wire>,
}

impl Circuit {
    pub fn wire(&self, name: SS) -> &Wire {
        &self.wires[name]
    }

    pub fn wire_mut(&mut self, name: SS) -> &mut Wire {
        self.wires.entry(name).or_insert(Wire::new(name))
    }

    pub fn expect_input_to_contain(&self, gate: &Gate, a: SS, b: SS, expected: impl Display) {
        if gate.inp.contains(&a) && gate.inp.contains(&b) {
            return;
        }
        panic!(
            "expect {expected}, gate inputs: [{}], wire a: {a}, wire b: {b}",
            gate.inp.iter().map(|&id| self.wire(id).name).join(","),
        );
    }

    pub fn expect_same_outputs(&self, a: SS, b: SS, ctx: impl Display) {
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
                        .filter(|&&i| i != wire.name)
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
        self.wires.values()
    }

    pub fn eval(&mut self, name: SS) -> usize {
        let wire = &self.wires[name];
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
        self.wire_mut(name).value = Some(value);
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
            circuit.wire_mut(name).value = Some(to_usize(value));
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
            let mut inp = [lhs, rhs];
            inp.sort();
            // maybe swap output of gate
            let out = swaps.get(out).copied().unwrap_or(out);
            let id = circuit.gates.alloc(Gate { inp, op, out });
            circuit.wire_mut(inp[0]).outputs.insert(id);
            circuit.wire_mut(inp[1]).outputs.insert(id);
            circuit.wire_mut(out).input = Some(id);
        }
        circuit
    }
}
