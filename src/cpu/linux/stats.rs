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

    // #[test]
    // fn cpustats_when_all_ten_then_33_percent() {
    //         let cpu_stat1 = "cpu  0 0 0 11743615 3155 0 3875 0 0 0";
    //         // user nice system idle
    //         let cpu_stat2 = "cpu  10 10 10 11797619 3156 0 3923 0 0 0";

    //         assert_eq!(cpu_busyness(cpu_stat1, cpu_stat2), Ok(33));
    // }

    // #[test]
    // fn when_cpu_stat_zero_then_idle_zero() {
    //         let cpu_stat = "cpu  0 0 0 11743615 3155 0 3875 0 0 0";

    //         assert_eq!(cpu_stats(&cpu_stat).unwrap().idle, 0);
    // }

    // #[test]
    // fn when_idle_in_cpu_stat_10_then_idle_10() {
    //         let cpu_stat = "cpu  3 5 10 11743615 3155 0 3875 0 0 0";

    //         assert_eq!(cpu_stats(&cpu_stat).unwrap().idle, 10);
    // }

    // #[test]
    // fn when_user_in_cpu_stat_3_then_idle_3() {
    //         let cpu_stat = "cpu  3 5 10 11743615 3155 0 3875 0 0 0";

    //         assert_eq!(cpu_stats(&cpu_stat).unwrap().user, 3);
    // }
}
