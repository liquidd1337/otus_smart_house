use smarthouse::smarthouse::*;
use smarthouse::smartroom::*;
use smarthouse::devices::*;

fn main() {
    // Инициализация устройств
    let socket = SmartSocket::default("Smart_socket".to_string());
    let socket_borrow = SmartSocket::default("Smart_socket".to_string());
    let thermo = SmartThermometer::default("Smart_thetmometr".to_string());
    //Инициализация комнат
    let kitchen = SmartRoom::default("kitchen".to_string(), socket.clone(), thermo.clone());
    let hall = SmartRoom::default("hall".to_string(), socket.clone(), thermo.clone());
    let living = SmartRoom::default("living".to_string(), socket.clone(), thermo.clone());
    let bathroom = SmartRoom::default("bathroom".to_string(), socket.clone(), thermo.clone());

    // Инициализация дома
    let house = SmartHouse::new("house".to_string(), kitchen, hall, living, bathroom);


    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider { socket: socket};
    
    let report1 = house.create_report(info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket_borrow,
        thermo: &thermo,
    };
    
    let report2 = house.create_report(info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
