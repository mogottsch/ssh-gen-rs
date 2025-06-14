# ssh-gen-rs

Generate SSH key pairs with custom suffixes in the public key.

```bash
# Install
cargo install --path .

# Usage
ssh-gen-rs <suffix> [-t <threads>]

# Examples
ssh-gen-rs deadbeef
ssh-gen-rs cafe -t 4
```
