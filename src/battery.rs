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
