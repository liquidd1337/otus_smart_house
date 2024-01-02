use thiserror::Error;

use crate::devices::*;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Error)]
pub enum SmartRoomError {
    #[error("Error while adding device: {0}")]
    AddDeviceError(#[from] DeviceError),
    #[error("Error while deleting device")]
    DeleteDeviceError(String),
}
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

    pub fn add_smart_device(&mut self, smart_device: Device) -> Result<(), SmartRoomError> {
        if let Ok(device_name) = smart_device.device_name() {
            self.smart_device.insert(device_name, smart_device);
            Ok(())
        } else {
            Err(SmartRoomError::AddDeviceError(smart_device.device_name().unwrap_err()))
        }
    }

    pub fn delite_device(&mut self, smart_device: Device) -> Result<(), SmartRoomError> {
        if let Ok(device_name) = smart_device.device_name() {
            self.smart_device.remove(&device_name);
            Ok(())
        } else {
            Err(SmartRoomError::DeleteDeviceError("The device does not exist".to_string()))
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
        let smart_room = SmartRoom::default("kitchen".to_string());
        assert!(!smart_room.room_name.is_empty());
    }

    #[test]
    fn add_smart_device() {
        let mut smart_room = SmartRoom::default("kitchen".to_string());
        let soket = Device::SmartSocket(SmartSocket::default("soket".to_string()));
        #[allow(dead_code)]
        smart_room.add_smart_device(soket);
        assert!(!smart_room.smart_device.is_empty());
    }

    #[test]
    fn delite_device() {
        let mut smart_room = SmartRoom::default("kitchen".to_string());
        let soket = Device::SmartSocket(SmartSocket::default("socket".to_string()));
        #[allow(dead_code)]
        smart_room.add_smart_device(soket.clone());
        assert!(!smart_room.smart_device.is_empty());
        smart_room.delite_device(soket);
        assert!(smart_room.smart_device.is_empty());
    }
}