use std::io;

enum PowerState {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerState {
    fn new(state: &str) -> Option<PowerState> {
        let state = state.trim().to_lowercase();
        match state.as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None,
        }
    }
}

fn print_power_action(state: PowerState) {
    use PowerState::*;
    match state {
        Off => println!("turning off..."),
        Sleep => println!("sleeping..."),
        Reboot => println!("rebooting..."),
        Shutdown => println!("shutting down..."),
        Hibernate => println!("hibernating..."),
    }
}

fn main() {
    println!("Enter any of the following option\nOff,\nSleep,\nReboot,\nShutdown,\nHibernate:");
    let mut input = String::new();
    let input_status = io::stdin().read_line(&mut input);
    if input_status.is_ok() {
        match PowerState::new(&input) {
            Some(state) => print_power_action(state),
            None => println!("invalid power state"),
        }
    } else {
        println!("error reading input")
    }
}
