# vanity-ssh-rs

Generate SSH key pairs with custom patterns in the public key.

```bash
# Install
cargo install --path .

# Usage
vanity-ssh-rs <pattern> [-t <threads>] [--ntfy <topic>]

# Examples
# Match suffix
vanity-ssh-rs yee

# Match regex pattern with ntfy notification
vanity-ssh-rs "/(?i)hello/" --ntfy mytopic
```

- `pattern`: The pattern to match in the public key. Use `/regex/` for regex patterns, otherwise matches suffix.
- `--ntfy <topic>`: Send a notification to the given [ntfy.sh](https://ntfy.sh) topic when a key is found (optional).
