use crate::gauge::bar;
use crate::status::StatusItem;
use sysctl::{Sysctl, CtlValue};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

const BATTERY:char = '🔋';

struct BatteryInfo {
    _info_expire: i32,
    _units: i32,
    _state: i32,
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
	_units: units,
	_state: state,
	_info_expire: info_expire
    })
}

fn minutes_to_human(min: i32) -> String {
    let hours = min / 60;
    let remainder = min - hours * 60;
    
    format!("{:02}h {:02}m", hours, remainder)
}

// Produce a string representation of BatteryInfo or render
// the error if something happens.
impl Display for BatteryInfo {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
	write!(f, "{}", 
	       u8::try_from(self.life).map_or_else(
		   |e| e.to_string(),
		   |life|
		   bar(life, 25).map_or_else(
		       |e| e.to_string(),
		       |battery_bar|
		       format!("{}% <span foreground=\\\"#00de55\\\" background=\\\"#555555\\\">{}</span> {}",
			       life,
			       battery_bar,
			       minutes_to_human(self.time)
		       )
		   )
	       )
	)
    }
}

pub fn battery_status() -> StatusItem {
    let mut battery_item = StatusItem::default();
    battery_item.markup = "pango".to_string();

    battery_item.name = "Battery".to_string();
    let battery_text = match read_battery_info() {
	Ok(info) => info.to_string(),
	Err(e) => e.to_string()
    };

    battery_item.full_text = format!("{}{}", BATTERY, battery_text);
    
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
