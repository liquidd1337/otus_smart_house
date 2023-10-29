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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SmartThermometer {
    pub name: String,
    status: bool,
    temperature: f32,
}

impl SmartThermometer {
    pub fn default(name : String) -> Self {
        Self {
            name,
            status: false,
            temperature: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_socket() {
        let socket = Device::SmartSocket(SmartSocket::default("Smart Socket".to_string()));
        assert_eq!(socket.device_name().is_some(), true);
    }

    #[test]
    fn default_thermo() {
        let thermo = Device::SmartThermometr(SmartThermometer::default("Smart thermometr".to_string()));

    }
}