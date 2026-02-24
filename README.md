# swaybar-status
A personalized Rust implementation for a swaybar status line on my Framework 16 laptop running Debian 13 and Sway.

This isn't designed as a drop-in solution--it's tailored to my hardware and setup. I'm sharing it as a reference for anyone building something similar.

## Example Output
Vol: ██████████▓▓▓▓▓▓▓▓▓▓ Mic: ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░     ✓     Tuesday, Feb 24 | 12:47     Full - 81%

## What it does
- **Volume**: Uses `wpctl` to query PipeWire
- **Network**: Queries NetworkManager via D-Bus
- **Time/Date**: Standard, should work on most systems
- **Battery**: Reads output from `/sys/class/power_supply/` (assumes laptop)

- Example shows fields in this order.
- Every field updates at a set frequency (default is 10Hz).

## Dependencies
- `nmcli` (NetworkManager)
- `wpctl` (PipeWire/WirePlumber)
- Standard Linux sysfs battery interface

## Notes
I'll respond to questions, concerns, or issues when able, but understand other projects take priority. If you're building something similar, feel free to reach out or share your approach!

Needs to be included in your sway config:
    *bar {
      status_command $PATH_TO_INSTALLATION/swaybar-status/target/release/swaybar-status
    }*

Then reload sway.
