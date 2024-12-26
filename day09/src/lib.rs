mod disk;

use disk::*;

fn compact_filesystem_fragmented(disk_map: &mut DiskMap) {
    if disk_map.files.is_empty() {
        return;
    }

    let (mut front_block, mut back_block) = disk_map.get_bounds().unwrap();

    while front_block < back_block {
        let Some(file) = disk_map.get_file(back_block) else {
            back_block -= 1;
            continue;
        };

        if let Some(file) = disk_map.get_file(front_block) {
            front_block += file.size();
            continue;
        }

        let file_size = file.size();
        let free_size = disk_map.get_free_size(front_block);
        assert!(free_size > 0);
        let size_to_move = free_size.min(file_size);

        disk_map.remove_file(file);
        disk_map.insert_file(File {
            id: file.id,
            first_block: front_block,
            last_block: front_block + size_to_move - 1,
        });

        let leftover_size = file_size - size_to_move;
        if leftover_size > 0 {
            disk_map.insert_file(File {
                id: file.id,
                first_block: file.first_block,
                last_block: file.first_block + leftover_size - 1,
            });
        }
    }
}

fn calculate_checksum_fragmented(disk_map: &DiskMap) -> u64 {
    (0..disk_map.size)
        .map(|block_no| (block_no, disk_map.get_file(block_no)))
        .take_while(|(_, file)| file.is_some())
        .map(|(block_no, file)| block_no as u64 * file.unwrap().id)
        .sum()
}

pub fn solve_part_1(input: &str) -> u64 {
    let mut disk_map: DiskMap = input.parse().expect("Failed to parse puzzle input");
    compact_filesystem_fragmented(&mut disk_map);
    calculate_checksum_fragmented(&disk_map)
}

fn compact_filesystem_defragmented(disk_map: &mut DiskMap) {
    if disk_map.files.is_empty() {
        return;
    }

    let (mut front_block, mut back_block) = disk_map.get_bounds().unwrap();

    while back_block > 0 {
        let Some(file) = disk_map.get_file(back_block) else {
            back_block -= 1;
            continue;
        };

        if let Some(file) = disk_map.get_file(front_block) {
            front_block += file.size();
            continue;
        }

        let file_size = file.size();
        let free_size = disk_map.get_free_size(front_block);
        assert!(free_size > 0);

        if back_block <= front_block {
            front_block = disk_map.get_bounds().unwrap().0;
            back_block = back_block.saturating_sub(file_size);
            continue;
        }

        if file_size > free_size {
            front_block += free_size;
            continue;
        }

        disk_map.remove_file(file);
        disk_map.insert_file(File {
            id: file.id,
            first_block: front_block,
            last_block: front_block + file_size - 1,
        });

        front_block = disk_map.get_bounds().unwrap().0;
        back_block -= file_size;
    }
}

fn calculate_checksum_defragmented(disk_map: &DiskMap) -> u64 {
    (0..disk_map.size)
        .map(|block_no| (block_no, disk_map.get_file(block_no)))
        .filter(|(_, file)| file.is_some())
        .map(|(block_no, file)| block_no as u64 * file.unwrap().id)
        .sum()
}

pub fn solve_part_2(input: &str) -> u64 {
    let mut disk_map: DiskMap = input.parse().expect("Failed to parse puzzle input");
    compact_filesystem_defragmented(&mut disk_map);
    calculate_checksum_defragmented(&disk_map)
}
