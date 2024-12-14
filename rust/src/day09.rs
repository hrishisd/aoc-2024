use std::collections::VecDeque;

static INPUT: [u8; 19_999] = parse_nums(*include_bytes!("../../inputs/day09/input-xin"));

fn main() {
    let total_disk_space: u64 = INPUT.iter().map(|i| *i as u64).sum();
    println!("total size: {total_disk_space}");
    println!("part 1: {}", part1(&INPUT));
    println!("part 2: {}", part2(&INPUT));
}

fn part1(input: &[u8]) -> usize {
    let mut nums = vec![0; input.len()];
    nums[..].clone_from_slice(input);
    let mut acc = 0;
    let mut out_idx = 0;
    let mut rear_data_idx = nums.len() - 1;
    let mut forward_idx = 0;

    while forward_idx <= rear_data_idx {
        assert!(
            forward_idx % 2 == 0,
            "We expect this to point to a data block"
        );
        // this is a data block
        while nums[forward_idx] > 0 {
            nums[forward_idx] -= 1;
            acc += forward_idx / 2 * out_idx;
            out_idx += 1;
        }
        // now we are done with data block
        forward_idx += 1;
        // now we populate the free space
        while nums[forward_idx] > 0 {
            // move from rear into free space
            while nums[rear_data_idx] == 0 && rear_data_idx > forward_idx {
                rear_data_idx -= 2;
            }
            // if there is no more rear data, break
            if nums[rear_data_idx] == 0 {
                break;
            }
            nums[rear_data_idx] -= 1;
            nums[forward_idx] -= 1;
            acc += rear_data_idx / 2 * out_idx;
            out_idx += 1;
        }
        forward_idx += 1;
    }
    acc
}

fn part2(nums: &[u8]) -> usize {
    #[derive(Debug, Clone, Copy)]
    struct Block {
        position: u32,
        size: u8,
    }
    let mut free_space: VecDeque<Block> = VecDeque::with_capacity(nums.len() / 2);
    let mut data_blocks: VecDeque<Block> = VecDeque::with_capacity(nums.len() / 2 + 1);
    data_blocks.push_back(Block {
        size: nums[0],
        position: 0,
    });
    for (idx, size) in nums.iter().enumerate().skip(1) {
        if idx % 2 == 0 {
            // data block
            let prev_free_block = free_space.back().unwrap();
            data_blocks.push_back(Block {
                position: prev_free_block.position + prev_free_block.size as u32,
                size: *size,
            });
        } else {
            let prev_data_block = data_blocks.back().unwrap();
            free_space.push_back(Block {
                position: prev_data_block.position + prev_data_block.size as u32,
                size: *size,
            });
        }
    }

    let mut acc = 0;
    // iterate through the data blocks with id in reverse order
    for (block_id, block) in data_blocks.iter().enumerate().rev() {
        let first_fit_idx = (0..free_space.len()).find(|idx| {
            free_space[*idx].size >= block.size && free_space[*idx].position < block.position
        });
        match first_fit_idx {
            Some(first_fit_idx) => {
                let free_block = &mut free_space[first_fit_idx];
                for i in 0..block.size {
                    acc += block_id * (free_block.position as usize + i as usize)
                }
                if free_block.size == block.size {
                    free_space.remove(first_fit_idx);
                } else {
                    let free_size = &mut free_block.size;
                    let position = &mut free_block.position;
                    *free_size -= block.size;
                    *position += block.size as u32;
                }
            }
            None => {
                // stays in same place
                for i in 0..block.size {
                    acc += block_id * (block.position as usize + i as usize)
                }
            }
        }
    }
    acc
}

const fn parse_nums<const N: usize>(arr: [u8; N]) -> [u8; N] {
    let mut res = [0; N];
    let mut i = 0;
    while i < N {
        res[i] = arr[i] - b'0';
        i += 1;
    }
    res
}
