# vanity-ssh-rs

Generate SSH key pairs with custom patterns in the public key.

```bash
# Install
cargo install vanity-ssh-rs

# Usage
vanity-ssh-rs <pattern1> [<pattern2> ...] [-t <threads>] [--ntfy <topic>]

# Examples
# Match any of several suffixes or regex patterns
vanity-ssh-rs yee woo "/(?i)hello/"

# Match with ntfy notification
vanity-ssh-rs yee woo --ntfy mytopic
```

- `pattern`: The pattern to match in the public key. Use `/regex/` for regex patterns, otherwise matches suffix. You can specify multiple patterns; any match will be accepted.
- `--ntfy <topic>`: Send a notification to the given [ntfy.sh](https://ntfy.sh) topic when a key is found (optional).
