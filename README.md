
# Rsoku

A cli roku remote for linux.

## Usage

When you call `rsoku` with no arguments, it enters keybind mode,
where keypresses are translated into actions.
You can also give rsoku a list of arguments to be run in script mode.
In script mode, arguments will be parsed into actions and run in one sequence.
For example, `rsoku r d s` will move right, move down, and select.
You can append a number to many actions to repeat the action,
eg `rsoku r2 d3` to move right twice and down 3 times.
Which actions can be repeated is documented [in this table](#actions-refrence)

## Configuration

The config file should be located at $XDG_CONFIG_HOME/rsoku/config.ron or $HOME/rsoku.ron.
There are two fields that need to be set:
addr, which is the ip address of the roku to send requests to,
and keybinds, which is a map from a tuple of a KeyCode
and bitflags for the modifier keys.
The KeyCode is a variant of the KeyCode enum of crossterm's event module,
and the bitflags are the underlying representation of crossterm's KeyModifiers struct.
The [crossterm docs](<https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html/>)
tell you everything you need to know about what KeyCodes are available.
The bits represent (from left to right) super, alt, control, shift.
So `0b0001` represents shift, `0b0010` represents control,
and `0b0011` represents control + shift.

## Actions Refrence

This table acts as a refrence for most actions you can bind to keys.
/ delineates a list of options, () separates a list of options from literal text.

| Action | Script Mode | HTTP | Repeat | Description |
|-|-|-|-|-|
| Up | u/up | keypress/Up | yes | move up |
| Down | d/down | keypress/Down | yes | move down |
| Left | l/left/> | keypress/Left | yes | move left |
| Right | r/right/> | keypress/Right | yes | move right |
| Select | s/select | keypress/Select | yes | select |
| Back | b/back | keypress/Back | yes | go back |
| Home | h/home | keypress/Home | no | go to home |
| Pause | p/pause | keypress/Play | no | play/pause |
| Rev | rev/reverse | keypress/Rev | yes | rewind |
| Fwd | fwd/forward | keypress/Fwd | yes | fast forward |
| InstantReplay | rw/replay | keypress/InstantReplay | yes | instant replay, aka rewind a few seconds |
| Info | i/info/* | keypress/Info | no | the * button |
| Backspace | x/backspace | keypress/Backspace | yes | backspace |
| VolumeUp | +/v+/volume+ | keypress/VolumeUp | yes | increase the volume |
| VolumeDown | -/v-/volume- | keypress/VolumeDown | yes | decrease the volume |
| Mute | m/mute/vx | keypress/Mute | yes | mute |
| PowerOff | o/off/power/poweroff | keypress/PowerOff | no | poweroff |
| ChannelUp | chu/channelup | keypress/ChannelUp | yes | move up a channel in live tv |
| ChannelDown | chd/channeldown | keypress/ChannelDown | yes | move down a channel in live tv |
| InputHDMI(1/2/3/4) | hmdi(1/2/3/4) | keypress/InputHDMI(1/2/3/4) | no | switch to hdmi port (1/2/3/4), eg `rsoku hdmi3` |
| InputAV1 | av | keypress/InputAV1 | no | switch to the AV port |

### KeyboardInput and the : command

The one special case is KeyboardInput, which when called with a keybind
will send your literal keypresses until escape is pressed.
In script mode, you can send a sequence of literal keypresses by prepending it with :,
eg `rsoku :text`; remember you must escape spaces for example `rsoku ':text with spaces'`.
