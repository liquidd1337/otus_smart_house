struct SmartHouse {
    house_name: String,
    smart_kitchen: SmartRoom,
    smart_hall: SmartRoom,
    smart_living_room: SmartRoom,
    smart_bathroom: SmartRoom,
}

impl SmartHouse {
    fn new(house_name: String) -> Self {
        Self {
            house_name,
            smart_kitchen: SmartRoom::default("Kitchen".to_string()),
            smart_hall: SmartRoom::default("Hall".to_string()),
            smart_living_room: SmartRoom::default("Living room".to_string()),
            smart_bathroom: SmartRoom::default("Bathroom".to_string()),
        }
    }

    fn get_rooms(&self) -> [String; 4] {
        [
            self.smart_kitchen.room_name.clone(),
            self.smart_hall.room_name.clone(),
            self.smart_living_room.room_name.clone(),
            self.smart_bathroom.room_name.clone(),
        ]
    }


    fn create_report(&self, provider: impl DeviceInfoProvider) -> String {
        let mut providers = Vec::new();
        for i in self.get_rooms() {
            providers.push(provider.device_info(i));
        }
        return format!(
            "House name : {}, 
                contains rooms {:?}.
                    The rooms have:
                    {:#?}",
            self.house_name,
            self.get_rooms(),
            providers,
        );
    }
}

struct SmartRoom {
    room_name: String,
    smart_socket: SmartSocket,
    smart_thermometr: SmartThermometer,
}

impl SmartRoom {
    fn default(room_name: String) -> Self {
        Self {
            room_name,
            smart_socket: SmartSocket::default(),
            smart_thermometr: SmartThermometer::default(),
        }
    }
}

trait DeviceInfoProvider {
    fn device_info(&self, room: String) -> String;
}

// ***** Пример использования библиотеки умный дом:

// Пользовательские устройства:
#[derive(Debug, Clone)]
struct SmartSocket {
    name: String,
    status: bool,
    voltage: f32,
}

impl SmartSocket {
    fn default() -> SmartSocket {
        Self {
            name: String::new(),
            status: false,
            voltage: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
struct SmartThermometer {
    name: String,
    status: bool,
    temperature: f32,
}

impl SmartThermometer {
    fn default() -> Self {
        Self {
            name: String::new(),
            status: false,
            temperature: 0.0,
        }
    }
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
#[derive(Debug)]
struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}
struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn device_info(&self, room: String) -> String {
        format!("{} contains: {} {}", room, room, &self.socket.name)
    }
}
impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn device_info(&self, room: String) -> String {
        format!(
            "{} contains : {} {} and {} {}",
            room, room, &self.socket.name, room, &self.thermo.name
        )
    }
}

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {
        name: "socket1".to_string(),
        status: true,
        voltage: 227.0,
    };
    let socket2 = SmartSocket {
        name: "socket2".to_string(),
        status: true,
        voltage: 227.0,
    };
    let thermo = SmartThermometer {
        name: "thermo".to_string(),
        status: true,
        temperature: 15.0,
    };

    // Инициализация дома
    let house = SmartHouse::new("HATA".to_string());

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report1 = house.create_report(info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house.create_report(info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
