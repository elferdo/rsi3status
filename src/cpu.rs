use crate::status::StatusItem;
use sysctl::{Sysctl, CtlValue};

const CPU:char = 'C';

fn get_sysctl_value(name: &str) -> Result<i32, sysctl::SysctlError> {
    let ctl = sysctl::Ctl::new(name)?;

    if let CtlValue::Int(value) = ctl.value()? {
	return Ok(value);
    } else {
	return Err(sysctl::SysctlError::UnknownType);
    }
}

fn read_cpu_freq() -> Result<i32, sysctl::SysctlError> {
    let freq = get_sysctl_value("dev.cpu.0.freq")?; 

    Ok(freq)
}

pub fn cpu_status() -> StatusItem {
    let mut item = StatusItem::default();

    item.full_text = read_cpu_freq().map_or_else(|e| e.to_string(), |freq| format!("{} {}Hz", CPU, freq));
    
    item
}
