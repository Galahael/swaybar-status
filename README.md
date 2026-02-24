# swaybar-status
---
A personalized Rust implementation for a swaybar status line on my Framework 16 laptop running Debian 13 + Sway.

This isn't designed as a drop-in solution - it's tailored to my hardware and setup. I'm sharing it as a reference for anyone building something similar.

## What it does
---
- **Volume**: Uses `wpctl` to query PipeWire
- **Network**: Queries NetworkManager via D-Bus
- **Time/Date**: Should work on most systems
- **Battery**: Reads directly from `/sys/class/power_supply/` (assumes laptop)

- Every field updates at a set frequency (default is 10Hz).
- Example below shows fields in that order.

## Dependencies
---
- `nmcli` (NetworkManager)
- `wpctl` (PipeWire/WirePlumber)
- Standard Linux sysfs battery interface

## Notes
---
I'll respond to questions, concerns, or issues when able, but understand other projects take priority. If you're building something similar, feel free to reach out or share your approach!

## Example Output:
---
Vol: ██████████▓▓▓▓▓▓▓▓▓▓ Mic: ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░     ✓     Tuesday, Feb 24 | 12:47     Full - 81%

- Note that the brightest shade of the ASCII bar indicates wpctl's overamplification feature. The "Vol" bar is currently reading 150%.
- Checkmark indicates current network connectivity.
