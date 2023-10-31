use std::fmt::Display;
#[derive(Debug, Clone)]
pub enum Device {
    SmartSocket(SmartSocket),
    SmartThermometr(SmartThermometer),
}

impl Device {
    pub fn device_name(&self) -> Option<String> {
        match self {
            Device::SmartSocket(smart_socket) => Some(smart_socket.name.clone()),
            Device::SmartThermometr(smart_thermometer) => Some(smart_thermometer.name.clone()),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
        assert!(socket.device_name().is_some());
    }

    #[test]
    fn default_thermo() {
        let thermo =
            Device::SmartThermometr(SmartThermometer::default("Smart thermometr".to_string()));
        assert!(thermo.device_name().is_some());
    }
}
