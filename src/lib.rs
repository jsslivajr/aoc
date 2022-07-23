use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Input {
    pub data: Vec<String>,
    pub lines: usize,
}

impl Input {
    pub fn new() -> Self {
        let data: Vec<String> = Vec::new();
        let lines: usize = 0;

        Self { data, lines }
    }

    pub fn read_file<P>(&mut self, file_path: P) -> Result<(), Box<dyn std::error::Error>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut line_count: usize = 0;
        for line in reader.lines() {
            if let Ok(line) = line {
                self.data.push(line);
                line_count += 1;
            }
        }
        self.lines = line_count;

        Ok(())
    }
}
