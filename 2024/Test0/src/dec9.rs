use std::cmp;
use std::collections::{HashMap, HashSet};
use crate::helpers::read_line;

fn defrag_checksum(input: &mut str) -> i64 {
    let data = unsafe { input.as_bytes_mut() };
    let mut current_size = data.len() as i32;
    if (current_size & 1) == 0 {
        data[(current_size - 1) as usize] = '.' as u8;
        current_size -= 1;
    }
    let mut checksum = 0i64;
    let mut input_pos = 0;
    let mut disk_pos = 0;
    let mut beg_file_idx = 0;
    let mut end_file_idx = data.len() as i32 / 2;
    let mut on_file = true;
    loop {
        // Handle file.
        if input_pos >= current_size {
            break;
        }

        let num_blocks = (data[input_pos as usize] as i32) - ('0' as u8 as i32);
        if (num_blocks < 0) || (num_blocks > 9) {
            panic!("Number should be between 0 and 9, but is: {}", num_blocks);
        }

        if on_file {
            for _i in 0..num_blocks {
                checksum += (disk_pos * beg_file_idx) as i64;
                disk_pos += 1;
            }
            data[input_pos as usize] = '.' as u8;
            input_pos += 1;
            beg_file_idx += 1;
            on_file = false;
        } else {
            let end_num_blocks = (data[(current_size - 1) as usize] as i32) - ('0' as u8 as i32);
            if (end_num_blocks < 0) || (end_num_blocks > 9) {
                panic!("Number should be between 0 and 9, but is: {}", num_blocks);
            }

            for _i in 0..cmp::min(num_blocks, end_num_blocks) {
                checksum += (disk_pos * end_file_idx) as i64;
                disk_pos += 1;
            }

            if end_num_blocks > num_blocks {
                data[(current_size - 1) as usize] -= num_blocks as u8;
                data[input_pos as usize] = '.' as u8;
                input_pos += 1;
                on_file = true;
            } else {
                data[(current_size - 1) as usize] = '.' as u8;
                data[(current_size - 2) as usize] = '.' as u8;
                data[input_pos as usize] -= end_num_blocks as u8;
                current_size -= 2;
                end_file_idx -= 1;
            }
        }
    }

    checksum
}

fn defrag_checksum2(input: &str) -> i64 {
    let mut files_by_size: HashMap<i32, Vec<i32>> = HashMap::new(); // Map from size to file_idxs.
    let mut block_chunks: Vec<i32> = Vec::new(); // List of file/empty sizes.
    let data = input.as_bytes();
    for i in 0..data.len() {
        let num_blocks = (data[i] as i32) - ('0' as u8 as i32);
        block_chunks.push(num_blocks);
        if i & 1 == 0 {
            // file
            if !files_by_size.contains_key(&num_blocks) {
                files_by_size.insert(num_blocks, Vec::new());
            }
            files_by_size.get_mut(&num_blocks).unwrap().push((i as i32) / 2);
        }
    }

    //println!("{:?}", block_chunks);
    //println!("{:?}", files_by_size);

    let mut outed_files: HashSet<i32> = HashSet::new();
    let mut checksum = 0i64;
    let mut disk_pos = 0;
    //let mut dbg: String = String::from("");
    let mut write = | fidx: i32, blocks: i32 | {
        for _i in 0..blocks {
            if fidx != -1 {
                checksum += (disk_pos * fidx) as i64;
                //dbg.push(('0' as i32 + fidx) as u8 as char);
            } else {
                //dbg.push('.');
            }
            disk_pos += 1;
        }
        //println!("{}", dbg);
    };
    for (idx, num_blocks) in block_chunks.iter().enumerate() {
        let on_empty = (idx & 1 != 0) || outed_files.contains(&((idx / 2) as i32));

        if !on_empty {
            let fidx = (idx as i32) / 2;
            write(fidx, *num_blocks);
            outed_files.insert(fidx);
            continue;
        }

        // Fill empty with last files that would fit into this empty.
        let mut blocks_left = *num_blocks;
        loop {
            if blocks_left <= 0 { break; }

            let mut best_fidx = -1;
            let mut best_size = -1;
            for cur_blocks in 1..blocks_left + 1 {
                let candidates = files_by_size.get_mut(&cur_blocks);
                if candidates.is_none() {
                    continue;
                }

                let candidates = candidates.unwrap();
                loop {
                    if candidates.is_empty() {
                        files_by_size.remove(&cur_blocks);
                        break;
                    }

                    let fidx = *candidates.last().unwrap();
                    if outed_files.contains(&fidx) {
                        candidates.pop();
                        continue;
                    }

                    if fidx > best_fidx {
                        best_fidx = fidx;
                        best_size = cur_blocks;
                    }
                    break;
                }
            }

            if best_fidx == -1 {
                break; // No candidates found.
            }

            files_by_size.get_mut(&best_size).unwrap().pop();
            write(best_fidx, best_size);
            outed_files.insert(best_fidx);
            blocks_left -= best_size;
        }

        write(-1, blocks_left);
    }

    checksum
}

#[allow(dead_code)]
pub(crate) fn dec9() {
    let mut input = read_line("dec9.in.txt").expect("Could not load input.");
    let result = defrag_checksum(input.as_mut_str());
    println!("{:?}", result);
}

#[allow(dead_code)]
pub(crate) fn dec9_2() {
    let mut input = read_line("dec9.in.txt").expect("Could not load input.");
    let result = defrag_checksum2(input.as_mut_str());
    println!("{:?}", result);
}
