use std::{
    cmp::Ordering,
    convert::Infallible,
    fmt::{Display, Formatter},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct File {
    pub id: u64,
    pub first_block: usize,
    pub last_block: usize,
}

impl File {
    pub fn size(&self) -> usize {
        self.last_block - self.first_block + 1
    }
}

pub struct DiskMap {
    pub files: Vec<File>,
    pub size: usize,
}

impl DiskMap {
    fn binary_search_files(&self, block_no: usize) -> Result<usize, usize> {
        self.files.binary_search_by(|file| {
            if (file.first_block..=file.last_block).contains(&block_no) {
                Ordering::Equal
            } else {
                file.first_block.cmp(&block_no)
            }
        })
    }

    pub fn get_file(&self, block_no: usize) -> Option<File> {
        let idx = self.binary_search_files(block_no).ok()?;

        Some(self.files[idx])
    }

    /// Returns number of free blocks after (and including) block at `block_no`.
    pub fn get_free_size(&self, block_no: usize) -> usize {
        let Err(idx) = self.binary_search_files(block_no) else {
            return 0; // Existing file was found at `block_no`, so there are no free blocks there.
        };
        let Some(file) = self.files.get(idx) else {
            return self.size - block_no + 1; // Free space until end of disk;
        };

        file.first_block - block_no
    }

    pub fn remove_file(&mut self, file: File) {
        let idx = self
            .binary_search_files(file.first_block)
            .expect("Cannot remove file not present on the disk");
        self.files.remove(idx);
    }

    pub fn insert_file(&mut self, file: File) {
        let idx = self
            .binary_search_files(file.first_block)
            .expect_err("Cannot insert file that overlaps with existing file(s)");
        self.files.insert(idx, file);
    }

    /// Returns numbers of lowest and highest allocated blocks.
    pub fn get_bounds(&self) -> Option<(usize, usize)> {
        Some((
            self.files.first()?.first_block,
            self.files.last()?.last_block,
        ))
    }
}

impl FromStr for DiskMap {
    type Err = Infallible;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let mut is_file = true;

        let mut disk_blocks = Vec::with_capacity(string.len() / 2 + 1);
        let mut disk_size = 0;
        let mut file_id = 0u64;
        for char in string.trim().chars() {
            let block_size = char.to_digit(10).unwrap() as usize;

            if is_file {
                disk_blocks.push(File {
                    id: file_id,
                    first_block: disk_size,
                    last_block: disk_size + block_size - 1,
                });
                file_id += 1;
            }

            disk_size += block_size;
            is_file = !is_file;
        }

        Ok(Self {
            files: disk_blocks,
            size: disk_size,
        })
    }
}

impl Display for DiskMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string: String = (0..self.size)
            .map(|block_no| self.get_file(block_no))
            .map(|file| match file {
                Some(file) => (file.id % 10).to_string(),
                None => ".".to_string(),
            })
            .collect();

        write!(f, "{}", string)
    }
}
