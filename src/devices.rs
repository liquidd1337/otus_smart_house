use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("The device doesn't have a name")]
    DeviceNotName(String),
}

#[derive(Debug, Clone, Error)]
pub enum Device {
    SmartSocket(SmartSocket),
    SmartThermometr(SmartThermometer),
}

impl Device {
    pub fn device_name(&self) -> Result<String, DeviceError>  {
        match self {
            Device::SmartSocket(smart_socket) => Ok(smart_socket.name.clone()),
            Device::SmartThermometr(smart_thermometer) => Ok(smart_thermometer.name.clone()),
        }
    }
}
impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Device::SmartSocket(smart_socket) => write!(f, "{}", smart_socket),
            Device::SmartThermometr(smart_thermometer) => write!(f, "{}", smart_thermometer),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Error)]
pub struct SmartSocket {
    pub name: String,
    status: bool,
    voltage: f32,
}

impl SmartSocket {
    pub fn default(name: String) -> SmartSocket {
        Self {
            name,
            status: false,
            voltage: 0.0,
        }
    }
}
impl Display for SmartSocket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SmartSocket name: {}, status: {}, voltage: {}",
            self.name, self.status, self.voltage
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Error)]
pub struct SmartThermometer {
    pub name: String,
    status: bool,
    temperature: f32,
}

impl SmartThermometer {
    pub fn default(name: String) -> Self {
        Self {
            name,
            status: false,
            temperature: 0.0,
        }
    }

}

impl Display for SmartThermometer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SmartThermo name: {}, status: {}, temperature: {}",
            self.name, self.status, self.temperature
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_socket() {
        let socket = Device::SmartSocket(SmartSocket::default("Smart Socket".to_string()));
        assert!(socket.device_name().is_ok());
    }

    #[test]
    fn default_thermo() {
        let thermo =
            Device::SmartThermometr(SmartThermometer::default("Smart thermometr".to_string()));
        assert!(thermo.device_name().is_ok());
    }
}
