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
        if (percentage - (iterator as f32 * 5.0)) as u8 > 100 {
            ascii_array.push('█');
        } else if (percentage / 5.0) as u8 > iterator as u8 {
            ascii_array.push('▓');
        } else {
            ascii_array.push('░');
        }
    }
    ascii_array.iter().collect()
}
