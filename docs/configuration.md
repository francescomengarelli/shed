# Configuration

`shed` has no configuration yet. When it does, it will follow the XDG Base
Directory spec:

- Config file: `$XDG_CONFIG_HOME/shed/` (default `~/.config/shed/`)
- Data: `$XDG_DATA_HOME/shed/` (default `~/.local/share/shed/`)
- State and logs: `$XDG_STATE_HOME/shed/` (default `~/.local/state/shed/`)

Precedence, highest first: command-line flags, environment variables
(`SHED_*`), config file, built-in defaults.
