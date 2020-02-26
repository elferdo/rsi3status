use super::stats::CpuStats;
use crate::gauge::bar;
use crate::status_item::StatusItem;
use std::convert::TryFrom;

pub fn cpu_busyness(_stat1: &CpuStats, _stat2: &CpuStats) -> Result<u8, String> {
    let total_elapsed_time: u32 =
        _stat2.user - _stat1.user + _stat2.nice - _stat1.nice + _stat2.system - _stat1.system
            + _stat2.idle
            - _stat1.idle;
    let elapsed_idle: u32 = _stat2.idle - _stat1.idle;

    let perc: u8 = u8::try_from(elapsed_idle * 100 / total_elapsed_time)
        .expect("A percentage value should always fit into a u8");

    Ok(perc)
}

pub fn read_cpu_freq() -> Result<i32, sysctl::SysctlError> {
    let freq = 100;

    Ok(freq)
}

fn cpu_stats(stats: &str) -> Result<CpuStats, ()> {
    let cpu_stats = CpuStats::new(&stats).unwrap();
    // let sum = cpu_stats.user + cpu_stats.system + cpu_stats.idle;

    // if sum == 0 {
    //     return Ok(CpuStats { idle: 0, user: 0 });
    // }

    Ok(CpuStats {
        user: 0,
        nice: 0,
        system: 0,
        idle: 0,
    })
}

pub fn cpu_status() -> StatusItem {
    let mut item = StatusItem::default();

    let life = 25;

    item.full_text = bar(life, 25).unwrap();

    item
}

#[cfg(test)]
mod test {
    use super::*;

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
