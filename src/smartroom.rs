use crate::devices::*;
use std::collections::HashMap;

pub struct SmartRoom {
    pub room_name: String,
    pub smart_device: HashMap<String, Device>,
}

impl SmartRoom {
    pub fn default(room_name: String) -> Option<SmartRoom> {
        Some(SmartRoom {
            room_name: String::new(),
            smart_device: HashMap::new(),
        })
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
            .unwrap()
            .smart_device
            .insert(String::from("Kitchen"), socket);
        assert_eq!(smart_room.is_some(), true);
    }
}
