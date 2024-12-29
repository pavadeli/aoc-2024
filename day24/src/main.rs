use circuit::{Circuit, Op};
use common::*;
use pathfinding::bfs;
use std::collections::BTreeMap;

mod circuit;

fn part1(input: SS) -> usize {
    let mut circuit = Circuit::parse(input);
    circuit
        .wires()
        .filter_map(|w| Some((w.name, w.as_z()?)))
        .collect_vec()
        .into_iter()
        .map(|(name, nr)| circuit.eval(name) << nr)
        .sum()
}

fn part2(input: SS) -> String {
    let circuit = Circuit::parse(input);

    // Try to find a circuit that contains a correct Ripple-carry adder which is
    // what we are looking for here.
    let mut result = bfs(
        &vec![],
        |swaps| {
            let new_swaps = check_correct_adders(&circuit, swaps)
                .err()
                .unwrap_or_default();
            new_swaps
                .into_iter()
                .combinations(2)
                .map(|c| {
                    let mut swaps = swaps.clone();
                    swaps.extend_from_slice(&c);
                    swaps
                })
                .collect_vec()
        },
        |swaps| check_correct_adders(&circuit, swaps).is_ok(),
    )
    .unwrap()
    .into_iter()
    .last()
    .unwrap();

    result.sort();
    result.join(",")
}

/// Returns `Ok` if this is THE circuit that we are looking for, otherwise `Err`
/// with a list of outputs that we can try to swap out given the current
/// circuit (if we can think of any at that point).
fn check_correct_adders(circuit: &Circuit, swaps: &[SS]) -> Result<(), Vec<SS>> {
    let mut circuit = circuit.clone();
    for (a, b) in swaps.iter().tuples() {
        circuit.swap_outs(a, b);
    }
    let mut outer_wires = circuit
        .wires()
        .filter_map(|w| {
            let (kind, nr) = w.as_io()?;
            Some((kind, (nr, w.name)))
        })
        .into_grouping_map()
        .collect::<BTreeMap<_, _>>();
    let x_wires = outer_wires.remove(&'x').unwrap();
    let y_wires = outer_wires.remove(&'y').unwrap();
    let z_wires = outer_wires.remove(&'z').unwrap();
    assert!(outer_wires.is_empty());

    // First evaluate the first half adder
    let (mut sum, mut carry) = eval_half_adder(&circuit, x_wires[&0], y_wires[&0])?;
    if sum != z_wires[&0] {
        return Err(vec![sum, z_wires[&0]]);
    }

    // Now continue into the ripple...
    for (&a, &b, &z) in multizip((x_wires.values(), y_wires.values(), z_wires.values())).skip(1) {
        (sum, carry) = eval_full_adder(&circuit, a, b, carry)?;
        if sum != z {
            return Err(vec![sum, z]);
        }
    }
    Ok(())
}

fn eval_full_adder(circuit: &Circuit, a: SS, b: SS, c: SS) -> Result<(SS, SS), Vec<SS>> {
    let (half_sum, half_carry) = eval_half_adder(circuit, a, b)?;

    let mut gates = circuit
        .output_gates(&circuit.wire(c).outputs)
        .into_group_map_by(|g| g.op);
    // expect one XOR gate to be connected to the carry wire
    let sum_gate = gates
        .remove(&Op::Xor)
        .and_then(|v| v.into_iter().exactly_one().ok())
        // In my case I did not need to investigate this further for any
        // "swappable" outputs, but this might be needed for other inputs (this
        // holds for all `Err(vec![])` outputs below).
        .ok_or(vec![])?;
    circuit.assert_input_contains(sum_gate, half_sum, c)?;
    // expect one AND gate to be connected to the carry wire
    let carry_gate = gates
        .remove(&Op::And)
        .and_then(|v| v.into_iter().exactly_one().ok())
        .ok_or(vec![])?;
    circuit.assert_input_contains(carry_gate, half_sum, c)?;
    // no other gates should be connected to the carry wire
    if !gates.is_empty() {
        return Err(vec![]);
    }

    let half_carry = circuit.wire(half_carry);
    circuit.assert_same_outputs(half_carry.name, carry_gate.out)?;
    // expect one final gate for the carry bit
    let carry_gate = circuit
        .output_gates(&half_carry.outputs)
        .exactly_one()
        .map_err(|_| vec![])?;
    if carry_gate.op != Op::Or {
        return Err(vec![]);
    }

    Ok((sum_gate.out, carry_gate.out))
}

fn eval_half_adder(circuit: &Circuit, a: SS, b: SS) -> Result<(SS, SS), Vec<SS>> {
    // Two lines into a half adder should always be connected to the same two
    // gates.
    circuit.assert_same_outputs(a, b)?;
    let a = circuit.wire(a);
    let mut gates = circuit.output_gates(&a.outputs).into_group_map_by(|g| g.op);
    // expect one XOR gate in a half adder
    let sum_gate = gates
        .remove(&Op::Xor)
        .and_then(|v| v.into_iter().exactly_one().ok())
        .ok_or(vec![])?;
    // expect one AND gate in a half adder
    let carry_gate = gates
        .remove(&Op::And)
        .and_then(|v| v.into_iter().exactly_one().ok())
        .ok_or(vec![])?;

    // no other gates should be connected to the half adder inputs
    if !gates.is_empty() {
        return Err(vec![]);
    }
    Ok((sum_gate.out, carry_gate.out))
}

boilerplate! {
    part1 => { test1 -> 4, test2 -> 2024, real -> 64755511006320 }
    part2 => { real -> "djg,dsd,hjm,mcq,sbg,z12,z19,z37" }
}
