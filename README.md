# Bloodborne Teleport CLI

A minimal command-line tool to teleport your character to Hunter's Dream in Bloodborne save files.

## Usage

```bash
teleport-to-hunters-dream <save_file>
```

### Arguments

- `<save_file>` - Path to your decrypted Bloodborne save file

## How It Works

### Save File Structure

Bloodborne save files contain player position data stored at a specific offset. The tool locates this data using a signature pattern (LCED - "Last Common Event Data"):

```
Offset +0x00 to +0x0B: Signature pattern [FF FF FF FF 00 00 00 00 00 00 00 00]
Offset +0x0C to +0x0F: X coordinate (f32 little-endian)
Offset +0x10 to +0x13: Y coordinate (f32 little-endian)
Offset +0x14 to +0x17: Z coordinate (f32 little-endian)
Offset +0x04 to +0x07: Map ID (u16 little-endian)
```

### Teleport Destination

The tool teleports the player to Hunter's Dream with the following fixed values:

| Property | Value |
|----------|-------|
| Map ID | 21 |
| X | -8.0 |
| Y | -6.0 |
| Z | -18.0 |

### Process Flow

1. **Parse Arguments** - CLI arguments are parsed using `clap` crate
2. **Read Save File** - Load the entire save file into memory
3. **Find LCED Pattern** - Search for the 12-byte signature that marks player position data
4. **Read Current Position** - Extract current coordinates and map ID for display
5. **Write New Position** - Replace coordinates and map ID with Hunter's Dream values
6. **Save File** - Write modified bytes back to disk

## Building

```bash
cd teleport-to-hunters-dream
cargo build --release
```

The binary will be located at `teleport-to-hunters-dream/target/release/teleport-to-hunters-dream`.

## Requirements

- Decrypted Bloodborne save file (not the original encrypted PS4 save)
- Rust toolchain (for building)

## Warning

- Always backup your save file before using this tool
- This tool modifies your save file in place
- Use only with decrypted save files from properly converted games

## License

This project is licensed under the [GPL-3.0 License](./LICENSE).
