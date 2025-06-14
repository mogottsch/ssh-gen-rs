# ssh-gen-rs

Generate SSH key pairs with custom patterns in the public key.

```bash
# Install
cargo install --path .

# Usage
ssh-gen-rs <pattern> [-t <threads>] [--ntfy <topic>]

# Examples
# Match suffix
ssh-gen-rs yee

# Match regex pattern with ntfy notification
ssh-gen-rs "/(?i)hello/" --ntfy mytopic
```

- `pattern`: The pattern to match in the public key. Use `/regex/` for regex patterns, otherwise matches suffix.
- `--ntfy <topic>`: Send a notification to the given [ntfy.sh](https://ntfy.sh) topic when a key is found (optional).
