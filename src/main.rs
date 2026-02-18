use std::io::{self, Write};
use std::thread;
use std::time::Duration;

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

pub mod time {
    use chrono::{Datelike, Local, Timelike, Weekday};

    pub fn present_time() -> String {
        let now = Local::now();

        let weekday = match now.weekday() {
            Weekday::Mon => "Monday",
            Weekday::Tue => "Tuesday",
            Weekday::Wed => "Wednesday",
            Weekday::Thu => "Thursday",
            Weekday::Fri => "Friday",
            Weekday::Sat => "Saturaday",
            Weekday::Sun => "Sunday",
        };

        let month = match now.month() {
            1 => "Jan",
            2 => "Feb",
            3 => "Mar",
            4 => "Apr",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "Aug",
            9 => "Sept",
            10 => "Oct",
            11 => "Nov",
            12 => "Dec",
            _ => unreachable!(),
        };

        let day = now.day();
        let hour = now.hour();

        // Formats a '0' in front of single-digit minutes:
        let minute = match now.minute() {
            x if x < 10 => format!("0{}", x),
            x if x >= 10 => format!("{}", x),
            _ => String::from("now.minute() error"),
        };

        format!("{}, {} {} | {}:{}", weekday, month, day, hour, minute)
    }
}

pub mod battery {
    use std::error::Error;
    use std::fmt;
    use std::fs;

    const DEFAULT_STATUS: ChargeState = ChargeState::Unknown;
    const DEFAULT_WATTS: f32 = 0.0;
    const DEFAULT_CAPACITY: u8 = 0;

    #[derive(Debug)]
    pub struct BatteryStatistics {
        pub status: ChargeState,
        pub watts: f32,
        pub capacity: u8,
    }
    impl BatteryStatistics {
        pub fn update(&mut self) {
            // Wattage is a function of both the current voltage and movement of current, thus, we need to poll both of these values:
            let voltage_now: f32 = fs::read_to_string("/sys/class/power_supply/BAT1/voltage_now")
                .ok()
                .and_then(|s| s.trim().parse().ok())
                .unwrap_or_default();

            let current_now: f32 = fs::read_to_string("/sys/class/power_supply/BAT1/current_now")
                .ok()
                .and_then(|s| s.trim().parse().ok())
                .unwrap_or_default();

            self.status = ChargeState::status().unwrap_or(DEFAULT_STATUS);
            self.capacity = fs::read_to_string("/sys/class/power_supply/BAT1/capacity")
                .ok()
                .and_then(|s| s.trim().parse().ok())
                .unwrap_or(DEFAULT_CAPACITY);
            self.watts = voltage_now * current_now / 1e12;
        }
    }
    impl fmt::Display for BatteryStatistics {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            if self.status == ChargeState::Full {
                write!(f, "{:?} - {:?}%", self.status, self.capacity)
            } else {
                write!(
                    f,
                    "{:?}: {:.1?}W - {:#?}%",
                    self.status, self.watts, self.capacity
                )
            }
        }
    }
    impl Default for BatteryStatistics {
        fn default() -> Self {
            Self {
                status: DEFAULT_STATUS,
                watts: DEFAULT_WATTS,
                capacity: DEFAULT_CAPACITY,
            }
        }
    }
    #[derive(Debug, PartialEq)]
    pub enum ChargeState {
        Full,
        Charging,
        Discharging,

        Anomolous,
        Unknown, //Default fallback behavior when file cannot be read.
    }
    impl ChargeState {
        pub fn status() -> Result<Self, Box<dyn Error>> {
            match fs::read_to_string("/sys/class/power_supply/BAT1/status")?.trim() {
                "Not charging" => Ok(ChargeState::Full),
                "Discharging" => Ok(ChargeState::Discharging),
                "Charging" => Ok(ChargeState::Charging),
                _ => Ok(ChargeState::Anomolous),
            }
        }
    }
}

pub mod volume {
    use std::error::Error;
    use std::fmt;
    use std::process::Command;

    const DEFAULT_VOLUME: f32 = 0.0;
    const DEFAULT_STATUS: bool = false;

    pub struct VolumeState {
        pub source_volume: f32,
        pub source_muted: bool,
        pub sink_volume: f32,
        pub sink_muted: bool,
    }

    impl VolumeState {
        pub fn update(&mut self) {
            let _ = self.try_update_source();
            let _ = self.try_update_sink();
        }

        fn try_update_source(&mut self) -> Result<(), Box<dyn Error>> {
            let output = Command::new("wpctl")
                .arg("get-volume")
                .arg("@DEFAULT_AUDIO_SOURCE@")
                .output()?;
            let query = String::from_utf8(output.stdout)?;
            self.source_muted = query.contains("MUTED");
            let parts: Vec<&str> = query.split_whitespace().collect();
            self.source_volume = parts[1].parse()?;
            Ok(())
        }

        fn try_update_sink(&mut self) -> Result<(), Box<dyn Error>> {
            let output = Command::new("wpctl")
                .arg("get-volume")
                .arg("@DEFAULT_AUDIO_SINK@")
                .output()?;
            let query = String::from_utf8(output.stdout)?;
            self.sink_muted = query.contains("MUTED");
            let parts: Vec<&str> = query.split_whitespace().collect();
            self.sink_volume = parts[1].parse()?;
            Ok(())
        }
    }
    impl Default for VolumeState {
        fn default() -> Self {
            Self {
                source_volume: DEFAULT_VOLUME,
                source_muted: DEFAULT_STATUS,
                sink_volume: DEFAULT_VOLUME,
                sink_muted: DEFAULT_STATUS,
            }
        }
    }
    impl fmt::Display for VolumeState {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let volume_display = ascii_bar(self.sink_volume);
            let mic_display = ascii_bar(self.source_volume);
            // write!(f, "Vol: {:.2}, Mic: {:.2}", self.sink, self.source)
            write!(f, "Vol: {} Mic: {}", volume_display, mic_display)
        }
    }

    fn ascii_bar(percentage: f32) -> String {
        let mut ascii_array = Vec::new();
        let percentage = percentage * 100.0;

        // Character Selection:
        // ▏ ▎ ▍ ▌ ▋ ▊ ▉ █    ▓ ░

        for iterator in 0..20 {
            if (percentage - (iterator as f32 * 2.5)) as u8 > 100 {
                ascii_array.push('▉');
            } else if (percentage / 5.0) as u8 > iterator as u8 {
                ascii_array.push('▓');
            } else {
                ascii_array.push('░');
            }
        }
        ascii_array.iter().collect()
    }
}

pub mod network {
    use std::error::Error;
    use std::fmt;
    use zbus::blocking::{Connection, Proxy};

    const DEFAULT_NETWORK: Connectivity = Connectivity::Unknown;

    pub struct NetworkState {
        connectivity: Connectivity,
    }
    impl NetworkState {
        pub fn update(&mut self, proxy: &Proxy) {
            let value: u32 = proxy.get_property("Connectivity").unwrap();
            self.connectivity = Connectivity::from_u32(value);
        }
    }
    impl fmt::Display for NetworkState {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let icon = match self.connectivity {
                Connectivity::Full => "✓",
                Connectivity::Limited => "⚠",
                Connectivity::Unknown => "?",
                _ => "✗",
            };
            write!(f, "{}", icon)
        }
    }
    impl Default for NetworkState {
        fn default() -> Self {
            Self {
                connectivity: DEFAULT_NETWORK,
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Connectivity {
        Full,
        Limited,
        Portal,
        None,
        Unknown,
    }
    impl Connectivity {
        fn from_u32(value: u32) -> Self {
            match value {
                4 => Connectivity::Full,
                3 => Connectivity::Limited,
                2 => Connectivity::Portal,
                1 => Connectivity::None,
                _ => Connectivity::Unknown,
            }
        }
    }

    pub fn setup_proxy() -> Result<Proxy<'static>, Box<dyn Error>> {
        let connection = Connection::system()?;
        let proxy = Proxy::new(
            &connection,
            "org.freedesktop.NetworkManager",
            "/org/freedesktop/NetworkManager",
            "org.freedesktop.NetworkManager",
        )?;
        Ok(proxy)
    }
}
