#[cfg(target_os = "freebsd")]
use sysctl::{CtlValue, Sysctl};

#[cfg(target_os = "freebsd")]
fn get_sysctl_value(name: &str) -> Result<i32, sysctl::SysctlError> {
        let ctl = sysctl::Ctl::new(name)?;

        if let CtlValue::Int(value) = ctl.value()? {
                return Ok(value);
        } else {
                return Err(sysctl::SysctlError::UnknownType);
        }
}

#[cfg(target_os = "freebsd")]
fn read_cpu_freq() -> Result<i32, sysctl::SysctlError> {
        let freq = get_sysctl_value("dev.cpu.0.freq")?;

        Ok(freq)
}
