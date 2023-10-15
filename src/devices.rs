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

#[derive(Debug, Clone)]
pub struct SmartThermometer {
    pub name: String,
    #[warn(dead_code)]
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
        let socket = SmartSocket::default("Smart_socket".to_string());
        assert_eq!(socket.name, "Smart_socket");
        assert_eq!(socket.status, false);
        assert_eq!(socket.voltage, 0.0);
    }

    #[test]
    fn default_thermo() {
        let thermo = SmartThermometer::default("Smart_thetmometr".to_string());
        assert_eq!(thermo.name, "Smart_thetmometr");
        assert_eq!(thermo.status, false);
        assert_eq!(thermo.temperature, 0.0);

    }
}