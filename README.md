## Ramixer

This project is the simplest TUI program based on [amixer](https://linux.die.net/man/1/amixer).  
I use `alsamixer` before, but it's not convenient enough. And its keybinding is weird.

So this is a nice replacement, only for me.

## Usage
Run `ramixer` without argument to open the TUI.  
The volume can also be changed directly from the command line, in which case
the program changes the volume, sends a `notify-send` notification and exits
without opening the TUI.

```sh
ramixer          # Open the TUI
ramixer -a 10    # Increase the master volume by 10%
ramixer -m 10    # Decrease the master volume by 10%
ramixer --help   # Show all options
```

`-a` (long form `--add`) and `-m` (long form `--minus`) take a value between
`1` and `100`, and cannot be used together.

### Notification
A single line with the new volume, plus [mako](https://mako-project.org/)'s
native progress bar drawn behind it:

```
52%
```

It is sent as:

```sh
notify-send --app-name RAmixer --hint int:value:52 52% ""
```

The empty body keeps the notification on a single line, and `value` is the
hint mako reads to draw the progress bar (`progress-color` sets its color).

## KeyBinding
| Keybinding         | Action                |
|--------------------|-----------------------|
| `Up` / `=`         | Increase 5% volume    |
| `Down` / `-`       | Decrease 5% volume    |
| `Shift+Up` / `+`   | Increase 10% volume   |
| `Shift+Down` / `_` | Decrease 10% volume   |
| `m`                | Mute/unmute Headphone |
| `M`                | Mute/unmute Speaker   |

## License
MIT
