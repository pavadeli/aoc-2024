use common::{SS, boilerplate};
use std::collections::VecDeque;

// solution: "simulate defrag" and calc result in one, by using a VecDeque, we
// can reduce the problem (/search) space incrementally (and drastically) by
// popping from both ends

#[derive(Debug)]
struct Block {
    file: Option<usize>,
    length: usize,
}

#[derive(Default)]
struct Checksum {
    disk_pos: usize,
    result: usize,
}

impl Checksum {
    fn file(&mut self, file: usize, length: usize) {
        self.result += file * (length * self.disk_pos + (length * (length - 1)) / 2);
        self.skip(length);
    }

    fn skip(&mut self, length: usize) {
        self.disk_pos += length;
    }

    fn result(&self) -> usize {
        self.result
    }
}

fn part1(input: SS) -> usize {
    go(input, |blocks, space| {
        blocks.back_mut().map(|block| {
            let length = block.length.min(space);
            block.length -= length;
            Block {
                file: block.file,
                length,
            }
        })
    })
}

fn part2(input: SS) -> usize {
    go(input, |blocks, space| {
        blocks
            .iter_mut()
            .rev()
            .find(|b| b.file.is_some() && b.length <= space)
            .map(|block| Block {
                file: block.file.take(),
                length: block.length,
            })
    })
}

// process the input as stated in the problem, `defrag_logic` is called with a
// relevant list of file-blocks, all blocks are guaranteed to:
// - not have been processed or checksummed yet
// - the one on the back is guaranteed to contain a file and be non-empty
fn go(input: SS, defrag_logic: fn(&mut VecDeque<Block>, usize) -> Option<Block>) -> usize {
    let mut blocks = parse(input);
    let mut cur_block = blocks.pop_front().unwrap();
    let mut checksum = Checksum::default();
    loop {
        // if we found a file, checksum it
        if let Some(file) = cur_block.file {
            checksum.file(file, cur_block.length);
            cur_block.length = 0;
        }

        // find the next non-empty block
        while cur_block.length == 0 {
            cur_block = match blocks.pop_front() {
                Some(next) => next,
                None => return checksum.result(), // if we are at the end, then we are done
            };
        }

        // check if we need to defrag, if there is a file, then we don't need
        // to defrag at this location
        if cur_block.file.is_some() {
            continue;
        }

        // Remove all garbage on the back of the block list.
        while blocks
            .back()
            .is_some_and(|b| b.file.is_none() || b.length == 0)
        {
            blocks.pop_back();
        }

        // ask for a file that fits in this empty space
        if let Some(defrag_block) = defrag_logic(&mut blocks, cur_block.length) {
            cur_block.length -= defrag_block.length;
            // if there is still space left in this empty space, push it back
            // onto the block list, we can do this efficiently because this is a
            // VecDeque where we popped at least one element from the front 🤩
            if cur_block.length > 0 {
                blocks.push_front(cur_block);
            }
            cur_block = defrag_block;
        } else {
            // if we could not find a file to defrag then skip this empty space
            // and continue with the next block
            checksum.skip(cur_block.length);
            cur_block.length = 0;
        }
    }
}

fn parse(input: &str) -> VecDeque<Block> {
    let mut id = 0;
    let mut free = false;
    input
        .bytes()
        .map(move |b| {
            let length = (b - b'0').into();
            let file = if free {
                None
            } else {
                let file = id;
                id += 1;
                Some(file)
            };
            free = !free;
            Block { file, length }
        })
        .collect()
}

boilerplate! {
    part1 => { test -> 1928, real -> 6337367222422 }
    part2 => { test -> 2858, real -> 6361380647183 }
}
