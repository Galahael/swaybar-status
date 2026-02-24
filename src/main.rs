use std::io::{self, Write};
use std::thread;
use std::time::Duration;

mod battery;
mod network;
mod time;
mod volume;

fn main() {
    use battery::*;
    use network::*;
    use time::*;
    use volume::*;

    let mut volume = VolumeState::default();
    let proxy = setup_proxy().unwrap();
    let mut network = NetworkState::default();
    let mut battery = BatteryStatistics::default();

    // As time is our only non-struct information, it is derived and formatted once every loop:
    let mut time;

    loop {
        volume.update();
        network.update(&proxy);
        time = present_time();
        battery.update();

        println!("{}     {}     {}     {}", volume, network, time, battery,);
        io::stdout().flush().unwrap();

        thread::sleep(Duration::from_millis(100));
    }
}
