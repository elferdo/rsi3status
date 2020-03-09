use super::stats::{CpuStats, CpuStatsProvider};
use crate::gauge::bar;
use crate::status::{StatusItem, StatusProvider};
use std::convert::TryFrom;
use std::fs::File;
use std::io::{BufRead, BufReader};

const CPU: char = '💻';

fn read_proc_stats_first_line() -> String {
    let filename = "/proc/stat";
    //    Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    let lines: Vec<_> = reader.lines().take(1).collect();

    lines[0].as_ref().unwrap().clone()
}

struct CpuStatusProvider {
    cpu_status_provider: CpuStatsProvider,
    cpu_status_item: StatusItem,
}

impl StatusProvider for CpuStatusProvider {
    fn update(&mut self) {
        let line = read_proc_stats_first_line();
        let stats = CpuStats::new(&line).unwrap();

        let idleness = self.cpu_status_provider.cpu_busyness(stats).unwrap();
        let life = 100 - idleness;

        let mut item = StatusItem::default();

        item.full_text = format!("{}{}", CPU, bar(life, 25).unwrap());

        self.cpu_status_item = item;
    }

    fn provide_status_item(&self) -> &StatusItem {
        &self.cpu_status_item
    }
}

pub fn cpu_status_provider() -> impl StatusProvider {
    CpuStatusProvider {
        cpu_status_provider: CpuStatsProvider::new(),
        cpu_status_item: StatusItem::default(),
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
