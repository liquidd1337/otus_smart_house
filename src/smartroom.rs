use crate::devices::*;
use std::{collections::HashMap, fmt::Display};
#[derive(Debug, Clone)]
pub struct SmartRoom {
    pub room_name: String,
    pub smart_device: HashMap<String, Device>,
}

impl SmartRoom {
    pub fn default(room_name: String) -> SmartRoom {
        SmartRoom {
            room_name,
            smart_device: HashMap::new(),
        }
    }

    pub fn add_smart_device(&mut self, smart_device: Device)  {
        if let Some(device_name) = smart_device.device_name() {
            self.smart_device.insert(device_name, smart_device);
        }
    }

    pub fn delite_device(&mut self, smart_device: Device) {
        if let Some(device_name) = smart_device.device_name() {
            self.smart_device.remove(&device_name);
        }
    }
}

impl Display for SmartRoom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} ", self.room_name)
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn default_room() {
        let mut smart_room = SmartRoom::default("kitchen".to_string());
        assert!(!smart_room.room_name.is_empty());
    }

    #[test]
    fn add_smart_device() {
        let mut smart_room = SmartRoom::default("kitchen".to_string());
        let soket = Device::SmartSocket(SmartSocket::default("soket".to_string()));
        smart_room.add_smart_device(soket);
        assert!(!smart_room.smart_device.is_empty());
    }

    #[test]
    fn delite_device() {
        let mut smart_room = SmartRoom::default("kitchen".to_string());
        let soket = Device::SmartSocket(SmartSocket::default("soket".to_string()));
        smart_room.add_smart_device(soket.clone());
        assert!(!smart_room.smart_device.is_empty());
        smart_room.delite_device(soket);
        assert!(smart_room.smart_device.is_empty());
    }
}
