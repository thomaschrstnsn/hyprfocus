# hyprfocus

Focus hyprland clients/windows based on matching title/class.
Will cycle between windows if client already focused.
Can also be configured to spawn a process (`hyprland dispatch exec`) if no matches are found.

Works similar to [Aero Traffic Control](https://github.com/thomaschrstnsn/aero-traffic-control) for Aerospace on macOS.

## Example usage

Meant to be bind to a key-combo in hyprland:

```
bind=$hyper, t, exec, hyprfocus --initial-class com.mitchellh.ghostty start ghostty
```

This will focus Ghostty or spawn a new instance if none is attached.
