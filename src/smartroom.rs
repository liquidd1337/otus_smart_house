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
}

impl Display for SmartRoom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.room_name)
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn default_room() {
        let mut smart_room = SmartRoom::default("kitchen".to_string());
        let socket = Device::SmartSocket(SmartSocket::default("Smart Socket".to_string()));
        smart_room
            .smart_device
            .insert(String::from("Kitchen socket"), socket);
        assert!(!smart_room.smart_device.is_empty());
    }
}
