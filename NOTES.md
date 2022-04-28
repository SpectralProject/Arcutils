# Notes

Cargo metadata is a command for manifest. You should specify metadata for arcutils on Neutron and Arcboot.
This allows arcutils to read the metadata for the current cargo project and interact with stuff like `cargo run`, `cargo build`, etc.

## I kinda like arcutils by itself

Better yet dont bother with cargo that much. Still try to support it but its kinda messy sometimes.

Just specify an `arcutils.yml` file to pass args to `arcutils run` and etc.
