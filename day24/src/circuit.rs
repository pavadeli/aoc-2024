use common::{Itertools, SS, second, to_usize};
use id_arena::{Arena, Id};
use std::collections::{BTreeSet, HashMap};

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

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Gate {
    pub inp: [SS; 2],
    pub op: Op,
    pub out: SS,
}

#[derive(Clone, Default)]
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

    pub fn assert_input_contains(&self, gate: &Gate, a: SS, b: SS) -> Result<(), Vec<SS>> {
        if gate.inp.contains(&a) && gate.inp.contains(&b) {
            return Ok(());
        }
        Err(gate.inp.iter().copied().chain([a, b]).unique().collect())
    }

    pub fn assert_same_outputs(&self, a: SS, b: SS) -> Result<(), Vec<SS>> {
        let a = &self.wire(a);
        let b = &self.wire(b);
        if a.outputs == b.outputs {
            return Ok(());
        }

        fn expected_from<'a>(
            wire: &'a Wire,
            this: &'a Circuit,
        ) -> impl Iterator<Item = SS> + use<'a> {
            wire.outputs.iter().map(|&g| {
                *this
                    .gate(g)
                    .inp
                    .iter()
                    .filter(|&&i| i != wire.name)
                    .exactly_one()
                    .unwrap()
            })
        }

        Err([a.name, b.name]
            .into_iter()
            .chain(expected_from(a, self))
            .chain(expected_from(b, self))
            .unique()
            .collect())
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

    pub fn parse(input: SS) -> Self {
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
            let id = circuit.gates.alloc(Gate { inp, op, out });
            circuit.wire_mut(inp[0]).outputs.insert(id);
            circuit.wire_mut(inp[1]).outputs.insert(id);
            circuit.wire_mut(out).input = Some(id);
        }
        circuit
    }

    pub fn swap_outs(&mut self, a: SS, b: SS) {
        let gate_id_b = self.wire(b).input.unwrap();
        let gate_id_a = self.wire_mut(a).input.replace(gate_id_b).unwrap();
        self.wire_mut(b).input = Some(gate_id_a);
        self.gates[gate_id_a].out = b;
        self.gates[gate_id_b].out = a;
    }
}
