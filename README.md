# Bloodborne Lantern Teleport

A tool to teleport your character to **any Lantern location** in Bloodborne save files. Available as both a **CLI** and an **interactive TUI** (Terminal User Interface).

## Features

- **Teleport to Any Lantern** - Choose from 44 lantern locations across all regions, including DLC areas
- **Interactive TUI** - Full terminal user interface with file browser, location search, and confirmation dialogs
- **CLI Support** - Command-line interface for quick operations or scripting
- **Save File Validation** - Validates save files before modification
- **Search & Filter** - Quickly find locations with fuzzy search (TUI)
- **Safe Operation** - Shows confirmation before modifying save files

## Installation

### From Source

```bash
git clone https://github.com/your-repo/lantern-teleport.git
cd lantern-teleport
cargo build --release
```

Binaries will be located at:
- CLI: `target/release/lantern-teleport-cli`
- TUI: `target/release/lantern-teleport-tui`

## Usage

### TUI (Recommended)

The TUI provides an interactive interface for browsing to your save file and selecting a destination:

```bash
lantern-teleport-tui
```

**TUI Features:**
- File browser to navigate to your save file
- Automatic validation of save files
- Locations grouped by region
- Search/filter locations by name (press `/`)
- Confirmation dialog before teleporting

**Key Bindings:**
| Key | Action |
|-----|--------|
| `↑`/`k` | Move up |
| `↓`/`j` | Move down |
| `Enter` | Select/Confirm |
| `Escape` | Go back/Cancel |
| `/` | Activate search |
| `q` | Quit |

### CLI

For quick operations or scripting:

```bash
# Teleport to Hunter's Dream (default)
lantern-teleport-cli path/to/userdata00XX

# Teleport to a specific location
lantern-teleport-cli path/to/userdata00XX --location "Central Yharnam"

# List all available locations
lantern-teleport-cli --list
```

#### CLI Arguments

| Argument | Description |
|----------|-------------|
| `<save_file>` | Path to your decrypted Bloodborne save file |
| `-l`, `--location <NAME>` | Destination location (supports fuzzy matching) |
| `--list` | List all available locations |

## Available Locations

The tool supports 44 lantern locations across 6 regions:

### Hunter's Dream
- Hunter's Dream

### Yharnam Headstone
- 1st Floor Sickroom
- Central Yharnam
- Great Bridge
- Tomb of Oedon
- Cathedral Ward
- Grand Cathedral Ward
- Upper Cathedral Ward
- Lumenflower Gardens
- Altar of Despair
- Old Yharnam
- Church of the Good Chalice
- Graveyard of the Darkbeast

### Frontier Headstone
- Hemwick Charnel Lane
- Witch's Abode
- Forbidden Woods
- Forbidden Grave
- Byrgenwerth
- Moonside Lake

### Unseen Headstone
- Yahar'gul, Unseen Village
- Yahar'gul Chapel
- Advent Plaza
- Hypogean Gaol
- Forsaken Castle Cainhurst
- Logarius' Seat
- Vileblood Queen's Chamber
- Abandoned Old Workshop

### Nightmare Headstone
- Lecture Building
- Lecture Building 2nd Floor
- Nightmare Frontier
- Nightmare of Mensis
- Mergo's Loft: Base
- Mergo's Loft: Middle
- Wet Nurse's Lunarium

### Hunter's Nightmare Headstone (DLC)
- Hunter's Nightmare
- Nightmare Church
- Nightmare Grand Cathedral
- Underground Corpse Pile
- Research Hall
- Lumenwood Garden
- Astral Clocktower
- Fishing Hamlet
- Lighthouse Hut
- Coast

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

### Process Flow

1. **Select Save File** - Browse and select your save file (TUI) or provide path (CLI)
2. **Validate** - Verify the file contains valid Bloodborne save data
3. **Select Destination** - Choose a lantern location
4. **Confirm** - Review and confirm the teleport operation
5. **Teleport** - Write new coordinates and map ID to the save file

## Requirements

- Decrypted Bloodborne save file (not the original encrypted PS4 save)
- Rust toolchain (for building from source)

## Warning

- **Always backup your save file before using this tool**
- This tool modifies your save file in place
- Use only with decrypted save files from properly converted games

## Project Structure

```
lantern_teleport/
├── crates/
│   ├── lantern-teleport-core/    # Core library
│   ├── lantern-teleport-cli/     # CLI application
│   └── lantern-teleport-tui/     # TUI application
└── Cargo.toml                    # Workspace configuration
```

## License

This project is licensed under the [GPL-3.0 License](./LICENSE).
