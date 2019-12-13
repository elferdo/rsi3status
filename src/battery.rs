use crate::gauge::bar;
use crate::status::StatusItem;
use sysctl::{Sysctl, CtlValue};
use std::convert::TryFrom;

const BATTERY:char = '🔋';

struct BatteryInfo {
    info_expire: i32,
    units: i32,
    state: i32,
    time: i32,
    life: i32
}

fn get_sysctl_value(name: &str) -> Result<i32, sysctl::SysctlError> {
    let ctl = sysctl::Ctl::new(name)?;

    if let CtlValue::Int(value) = ctl.value()? {
	return Ok(value);
    } else {
	return Err(sysctl::SysctlError::UnknownType);
    }
}

fn read_battery_info() -> Result<BatteryInfo, sysctl::SysctlError> {
    let life = get_sysctl_value("hw.acpi.battery.life")?; 
    let time = get_sysctl_value("hw.acpi.battery.time")?;
    let state = get_sysctl_value("hw.acpi.battery.state")?;
    let info_expire = get_sysctl_value("hw.acpi.battery.info_expire")?;
    let units = get_sysctl_value("hw.acpi.battery.units")?;

    Ok(BatteryInfo{
	life: life,
	time: time,
	units: units,
	state: state,
	info_expire: info_expire
    })
}

fn minutes_to_human(min: i32) -> String {
    let hours = min / 60;
    let remainder = min - hours * 60;
    
    format!("{:02}h {:02}m", hours, remainder)
}

pub fn battery_status() -> StatusItem {
    let mut battery_item = StatusItem::default();
    battery_item.markup = "pango".to_string();

    battery_item.name = "Battery".to_string();
    battery_item.full_text = format!("{}{}", BATTERY, "error reading battery info");

    if let Ok(battery_info) = read_battery_info() {
	if let Ok(life) = u8::try_from(battery_info.life) {
	    if let Ok(battery_bar) = bar(life, 25) {
		battery_item.full_text =
		    format!("{}{}% <span foreground=\\\"#00de55\\\" background=\\\"#555555\\\">{}</span> {}",
			    BATTERY,
			    life,
			    battery_bar,
			    minutes_to_human(battery_info.time)
		    );
	    }
	}
    }

    battery_item
}


#[cfg(test)]
mod test {
    use super::minutes_to_human;
    
    #[test]
    fn sixty_minutes() {
	assert_eq!(minutes_to_human(60), "01h 00m");
    }
}
