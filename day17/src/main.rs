use common::*;
use std::iter;

#[derive(Debug)]
struct Cpu {
    a: usize,
    b: usize,
    c: usize,
    pc: usize,
    program: &'static [usize],
}

const ADV: usize = 0;
const BDV: usize = 6;
const CDV: usize = 7;
const BXL: usize = 1;
const BST: usize = 2;
const JNZ: usize = 3;
const BXC: usize = 4;
const OUT: usize = 5;

impl Cpu {
    fn combo(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid combo operand: {operand:?}"),
        }
    }

    fn div(&self, operand: usize) -> usize {
        self.a / 2_usize.pow(self.combo(operand) as u32)
    }

    fn exec_op(&mut self) -> Option<usize> {
        let opcode = self.program[self.pc];
        let operand = self.program[self.pc + 1];
        self.pc += 2;
        match opcode {
            // divs
            ADV => self.a = self.div(operand),
            BDV => self.b = self.div(operand),
            CDV => self.c = self.div(operand),

            BXL => self.b ^= operand,
            BST => self.b = self.combo(operand) % 8,
            JNZ => {
                if self.a != 0 {
                    self.pc = operand
                }
            }
            BXC => self.b ^= self.c,
            OUT => return Some(self.combo(operand) % 8),

            _ => panic!("invalid opcode {opcode:?}"),
        }
        None
    }

    fn exec_program(mut self) -> impl Iterator<Item = usize> {
        iter::from_fn(move || {
            while self.pc < self.program.len() {
                let result = self.exec_op();
                if let Some(output) = result {
                    return Some(output);
                }
            }
            None
        })
    }

    fn parse(mut input: SS) -> Self {
        let mut read = |prefix| {
            let (a, b) = input
                .strip_prefix(prefix)
                .unwrap()
                .split_once('\n')
                .unwrap();
            input = b;
            a
        };

        Self {
            a: to_usize(read("Register A: ")),
            b: to_usize(read("Register B: ")),
            c: to_usize(read("Register C: ")),
            pc: 0,
            program: input
                .trim()
                .strip_prefix("Program: ")
                .unwrap()
                .split(',')
                .map(to_usize)
                .collect_vec()
                .leak(),
        }
    }

    fn with_a(&self, a: usize) -> Self {
        Cpu { a, ..*self }
    }
}

fn part1(input: SS) -> String {
    Cpu::parse(input).exec_program().join(",")
}

fn part2a(input: SS) -> usize {
    let cpu = Cpu::parse(input);
    let program = cpu.program;
    (0..usize::MAX)
        .find(|&a| cpu.with_a(a).exec_program().eq(program.iter().copied()))
        .unwrap()
}

/*
We can simply search for the solution for the test-case, but it takes
too long to find the solution for our puzzle input, so let's analyze our
program and see what we can find out..

Our program:
2,4: bst A: B = (A % 8)
1,1: bxl 1: B ^= 1
7,5: cdv B: C = A / (2^B)
1,5: bxl 5: B ^= 5
4,0: bxc .: B ^= C
5,5: out B: output(B % 8)
0,3: adv 3: A = A / 8
3,0: jnz 0: start over if A != 0

Which is something like:
10 output(((A % 8) ^ 1 ^ 5 ^ (A / (2^((A % 8) ^ 1)))) % 8)
20 A = A / 8
30 if A != 0 goto 10

If we take this expression and simplify it a bit:
((A % 8) ^ 1 ^ 5 ^ (A / (2^((A % 8) ^ 1)))) % 8

We are allowed to distribute the modulo operations over the XOR operations
(because (% 8) === (& 7) and & is distributive over ^), so we get:
(A % 8) ^ 4 ^ (A / (2^((A % 8) ^ 1)) % 8)

Let X be the last 3 bits of A, i.e. A % 8.
X ^ 4 ^ (A / (2^(X^1)) % 8)

Which is:
X ^ 4 ^ ((A >> (X^1)) & 7)

Then we see clearly that each output depends on:
- the last 3 bits of the A register: (X)
- some other 3 bits of the A register: ((A >> (X^1)) & 7)

The second part is the most troublesome, (X ^ 1) is used as kind of a lookup
into the rest of the A register. So the outputs are not independent. Of course
not, why would they, that would have been too easy, I guess. 😠

Because we only have 8 possible values of X, lets make a table to wrap our
head around it.

X | X^1 | when A < 8, outputs:
0 |  1  |     4
1 |  0  | always outputs 4 (the only independent value)
2 |  3  |     6
3 |  2  |     7
4 |  5  |     0
5 |  4  |     1
6 |  7  |     2
7 |  6  |     3

So we can limit our search space now to
from:   0b100 << (15 * 3)
to:     0b101 << (15 * 3)

But that is still way too large!!!

Ok, let's go a step further, can we also do the next number? (3)
Given 0b100...
X | X^1 | given A has pattern 0b100xxx, outputs:
0 |  1  |     4
1 |  0  | always outputs 4 (the only independent value)
2 |  3  |     2
3 |  2  |     7
4 |  5  |     1
5 |  4  |     3
6 |  7  |     2
7 |  6  |     3

AHA! So, now we have two possibilities. This starts to look like a tree search
and we know how to do that!
*/

fn part2b(input: SS) -> usize {
    let cpu = Cpu::parse(input);
    let program = cpu.program;
    let mut possibilities = vec![0b100_usize];
    for start in (0..program.len() - 1).rev() {
        let tail = &program[start..];
        possibilities = possibilities
            .into_iter()
            .cartesian_product(0..8)
            .map(|(a, x)| (a << 3) + x)
            .filter(|a| cpu.with_a(*a).exec_program().eq(tail.iter().copied()))
            .collect();
    }
    possibilities.into_iter().min().unwrap()
}

boilerplate! {
    part1 => { test -> "4,6,3,5,6,3,5,2,1,0", real -> "4,1,7,6,4,1,0,2,7" }
    part2a => { test2 -> 117440 }
    part2b => { real -> 164279024971453 }
}
