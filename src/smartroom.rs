use crate::devices::*;
pub struct SmartRoom {
    pub room_name: String,
    pub smart_socket: SmartSocket,
    pub smart_thermometr: SmartThermometer,
}

impl SmartRoom {
    pub fn default(room_name: String, smart_socket: SmartSocket, smart_thermometr: SmartThermometer) -> Self {
        Self {
            room_name,
            smart_socket,
            smart_thermometr,
        }
    }
}

#[cfg(test)]
mod tests {
    

    use super::*;

    #[test]
    fn default_room() {
        let socket = SmartSocket::default("Smart_socket".to_string());
        let thermo = SmartThermometer::default("Smart_thetmometr".to_string());
        let smart_room = SmartRoom::default("kitchen".to_string(), socket, thermo);
        assert_eq!(smart_room.room_name, "kitchen");
    }
}

