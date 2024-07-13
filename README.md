# Pomodoro
This is a simple pomodoro timer.

To use it, call it as follows

`$ cargo run -- <Loop?> <Time> <Label> ...`

You can have as many `<Time> <Label>` pairs as you want, but each time must be followed by a label.

For multi-word labels, use quote marks:

`$ cargo run -- 25 "Work on A" 5 Break 25 "Work on B" 5 Break`

`<Time>` can be in the format:

- `#s` for # seconds
- `#m` for # minutes (this is the default if no unit is specified)
- `#h` for # hours

## Examples

### With cargo:

50 minutes of work, then 5 minutes of break, then another 50 minutes working, followed by a 15 minute break.

`$ cargo run -- loop 50m Work 5m Break 50m Work 15m Break`

25 minutes work followed by 5 minutes of break

`$ cargo run -- 25 Work 5 Break`

### Calling the binary directly:

50 minutes of work, then 5 minutes of break, then another 50 minutes working, followed by a 15 minute break.

`$ ./pomodoro loop 50m Work 5m Break 50m Work 15m Break`

25 minutes work followed by 5 minutes of break

`$ ./pomodoro 25 Work 5 Break`

# Known problems
## Currently working on

- No validation for time lengths(i.e. negative duration)

## Not working on
