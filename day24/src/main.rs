use circuit::{Circuit, Op, Wire, WireKind};
use common::*;
use id_arena::Id;
use std::collections::BTreeMap;

mod circuit;

fn part1(input: SS) -> usize {
    let mut circuit = Circuit::parse(input, &[]);
    circuit
        .wires()
        .filter_map(|w| Some((w.id, w.name.as_z()?)))
        .collect_vec()
        .into_iter()
        .map(|(id, nr)| circuit.eval(id) << nr)
        .sum()
}

/// The output of this program will show problems it encountered during analysis
/// of the logic. The error messages, though cryptic at best, should help you
/// find the swaps pretty easily.
fn part2(input: SS, swaps: &[(SS, SS)]) -> String {
    let circuit = Circuit::parse(input, swaps);
    let mut outer_wires = circuit
        .wires()
        .filter_map(|w| match w.name {
            WireKind::X(nr) => Some(('x', (nr, w.id))),
            WireKind::Y(nr) => Some(('y', (nr, w.id))),
            WireKind::Z(nr) => Some(('z', (nr, w.id))),
            WireKind::Other(_) => None,
        })
        .into_grouping_map()
        .collect::<BTreeMap<_, _>>();
    let x_wires = outer_wires.remove(&'x').unwrap();
    let y_wires = outer_wires.remove(&'y').unwrap();
    let z_wires = outer_wires.remove(&'z').unwrap();
    assert!(outer_wires.is_empty());

    // Now evaluate the first half adder
    println!("evaluating first half adder");
    let (mut sum, mut carry) = eval_half_adder(&circuit, x_wires[&0], y_wires[&0]);
    circuit.expect_same_wires(sum, z_wires[&0], "output of first half adder");

    for (i, (&a, &b, &z)) in multizip((x_wires.values(), y_wires.values(), z_wires.values()))
        .enumerate()
        .skip(1)
    {
        println!("evaluating full adder {i}");
        (sum, carry) = eval_full_adder(&circuit, a, b, carry);
        circuit.expect_same_wires(sum, z, format!("output of full adder {i}"));
    }

    swaps.iter().flat_map(|(a, b)| [a, b]).sorted().join(",")
}

fn eval_full_adder(
    circuit: &Circuit,
    a: Id<Wire>,
    b: Id<Wire>,
    c: Id<Wire>,
) -> (Id<Wire>, Id<Wire>) {
    let (half_sum, half_carry) = eval_half_adder(circuit, a, b);

    let mut gates = circuit
        .output_gates(&circuit.wire(c).outputs)
        .into_group_map_by(|g| g.op);
    let sum_gate = gates
        .remove(&Op::Xor)
        .and_then(|v| v.into_iter().exactly_one().ok())
        .expect("expect one XOR gate to be connected to the carry wire");
    circuit.expect_input_to_contain(
        sum_gate,
        half_sum,
        c,
        "expect one XOR gate to be connected to both the carry and the half-sum wire",
    );
    let carry_gate = gates
        .remove(&Op::And)
        .and_then(|v| v.into_iter().exactly_one().ok())
        .expect("expect one AND gate to be connected to the carry wire");
    circuit.expect_input_to_contain(
        carry_gate,
        half_sum,
        c,
        "expect one AND gate to be connected to both the carry and the half-sum wire",
    );
    assert!(
        gates.is_empty(),
        "no other gates should be connected to the carry wire"
    );

    let half_carry = circuit.wire(half_carry);
    circuit.expect_same_outputs(half_carry.id, carry_gate.out, "half-carry vs carry-gate");
    let carry_gate = circuit
        .output_gates(&half_carry.outputs)
        .exactly_one()
        .ok()
        .expect("expect one final gate for the carry bit");
    assert_eq!(carry_gate.op, Op::Or);

    (sum_gate.out, carry_gate.out)
}

fn eval_half_adder(circuit: &Circuit, a: Id<Wire>, b: Id<Wire>) -> (Id<Wire>, Id<Wire>) {
    // Two lines into a half adder should always be connected to the same two
    // gates.
    circuit.expect_same_outputs(a, b, "inputs of half adder");
    let a = circuit.wire(a);
    let mut gates = circuit.output_gates(&a.outputs).into_group_map_by(|g| g.op);
    let sum_gate = gates
        .remove(&Op::Xor)
        .and_then(|v| v.into_iter().exactly_one().ok())
        .expect("expect one XOR gate in a half adder");
    let carry_gate = gates
        .remove(&Op::And)
        .and_then(|v| v.into_iter().exactly_one().ok())
        .expect("expect one AND gate in a half adder");

    assert!(
        gates.is_empty(),
        "no other gates should be connected to the half adder inputs"
    );
    (sum_gate.out, carry_gate.out)
}

const SWAPS: &[(&str, &str); 4] = &[
    ("z12", "djg"),
    ("sbg", "z19"),
    ("hjm", "mcq"),
    ("dsd", "z37"),
];

boilerplate! {
    part1 => { test1 -> 4, test2 -> 2024, real -> 64755511006320 }
    part2 => { real(SWAPS) -> "djg,dsd,hjm,mcq,sbg,z12,z19,z37" }
}
