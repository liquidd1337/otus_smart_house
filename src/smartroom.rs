use std::collections::HashMap;
use crate::devices::*;

pub struct SmartRoom {
    pub room_name: String,
    pub smart_device: HashMap<String, Device>,
}

impl SmartRoom {
    pub fn default(room_name: String) -> Result<SmartRoom, &'static str> {
        match room_name {
            ref String => {
                 Ok (SmartRoom {
                    room_name,
                    smart_device: HashMap::new(),
                })
            },
            _ => Err("invalid room name"),
        }
    }
}

#[cfg(test)]
mod tests {
    

    use super::*;

    #[test]
    fn default_room() {
        let smart_room = SmartRoom::default("kitchen".to_string());
        let smart_thermo = Device::Smart_socket(())
        smart_room.smart_device.insert(String::from("Kithen"), )
        assert_eq!(smart_room.is_ok(), true);
    }
}

