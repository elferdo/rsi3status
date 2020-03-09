use std::convert::TryFrom;
use std::default::Default;

#[derive(Default)]
pub struct CpuStats {
    pub user: u32,
    pub nice: u32,
    pub system: u32,
    pub idle: u32,
}

impl CpuStats {
    pub fn new(stats: &str) -> Option<CpuStats> {
        if stats.is_empty() {
            return None;
        }

        let _items: Vec<_> = stats.split_whitespace().collect();

        let user = _items[1].parse::<u32>().unwrap();
        let nice = _items[2].parse::<u32>().unwrap();
        let system = _items[3].parse::<u32>().unwrap();
        let idle = _items[4].parse::<u32>().unwrap();

        Some(CpuStats {
            user: user,
            nice: nice,
            system: system,
            idle: idle,
        })
    }
}

fn cpu_busyness(_stat1: &CpuStats, _stat2: &CpuStats) -> Result<u8, String> {
    let total_elapsed_time: u32 =
        _stat2.user - _stat1.user + _stat2.nice - _stat1.nice + _stat2.system - _stat1.system
            + _stat2.idle
            - _stat1.idle;
    let elapsed_idle: u32 = _stat2.idle - _stat1.idle;

    let perc: u8 = u8::try_from(elapsed_idle * 100 / total_elapsed_time)
        .expect("A percentage value should always fit into a u8");

    Ok(perc)
}

pub struct CpuStatsProvider {
    last_stats: CpuStats,
}

impl CpuStatsProvider {
    pub fn new() -> Self {
        CpuStatsProvider {
            last_stats: CpuStats::default(),
        }
    }

    pub fn cpu_busyness(&mut self, stats: CpuStats) -> Result<u8, String> {
        let result = cpu_busyness(&self.last_stats, &stats);

        self.last_stats = stats;

        return result;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cpustats_when_valid_then_result_matches() {
        let user = 19203;
        let nice = 1039;
        let system = 7739;
        let idle = 2230629;
        let stat = format!(
            "cpu  {} {} {} {} 1237 0 1533 0 0 0",
            user, nice, system, idle
        );

        let cpu_stats = CpuStats::new(&stat).unwrap();
        assert_eq!(cpu_stats.user, user);
        assert_eq!(cpu_stats.nice, nice);
        assert_eq!(cpu_stats.system, system);
        assert_eq!(cpu_stats.idle, idle);
    }

    #[test]
    fn cpustats_when_empty_then_none() {
        let cpu_stats = CpuStats::new("");
        assert!(cpu_stats.is_none());
    }

    #[test]
    fn cpustats_when_equal_and_called_twice_then_25() {
        let cpu_stat1 = CpuStats::new("cpu  3 3 3 3 3155 0 3875 0 0 0").unwrap();
        let cpu_stat2 = CpuStats::new("cpu  8 8 8 8 3155 0 3875 0 0 0").unwrap();
        let mut provider = CpuStatsProvider::new();

        assert_eq!(provider.cpu_busyness(cpu_stat1), Ok(25));
        assert_eq!(provider.cpu_busyness(cpu_stat2), Ok(25));
    }

    #[test]
    fn cpustats_when_idle_double_and_called_twice_then_50() {
        let cpu_stat1 = CpuStats::new("cpu  3 3 3 3 3155 0 3875 0 0 0").unwrap();
        let cpu_stat2 = CpuStats::new("cpu  8 8 8 18 3155 0 3875 0 0 0").unwrap();
        let mut provider = CpuStatsProvider::new();

        assert_eq!(provider.cpu_busyness(cpu_stat1), Ok(25));
        assert_eq!(provider.cpu_busyness(cpu_stat2), Ok(50));
    }

    #[test]
    fn cpu_busyness_when_real_values_then_real_percentage() {
        let user1 = 132979;
        let nice1 = 37383;
        let system1 = 39062;
        let idle1 = 11743615;

        let user2 = 133486;
        let nice2 = 37383;
        let system2 = 39218;
        let idle2 = 11797619;

        let stats1 = format!(
            "cpu  {} {} {} {} 3155 0 3875 0 0 0",
            user1, nice1, system1, idle1
        );
        let stats2 = format!(
            "cpu  {} {} {} {} 3156 0 3923 0 0 0",
            user2, nice2, system2, idle2
        );

        let cpu_stats1 = CpuStats::new(&stats1).unwrap();
        let cpu_stats2 = CpuStats::new(&stats2).unwrap();

        let total_elapsed_time: u32 =
            user2 - user1 + nice2 - nice1 + system2 - system1 + idle2 - idle1;
        let elapsed_idle: u32 = idle2 - idle1;

        let perc: u8 = u8::try_from(elapsed_idle * 100 / total_elapsed_time).unwrap();

        assert_eq!(cpu_busyness(&cpu_stats1, &cpu_stats2).unwrap(), perc);
    }
}
