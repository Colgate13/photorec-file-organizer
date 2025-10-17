# PhotoRec File Organizer

A simple and efficient CLI tool written in Rust to organize files recovered by PhotoRec.

## Features

- **Remove small files**: Automatically delete files smaller than 10KB
- **Organize by extension**: Sort files into folders based on their file type
- **Handle duplicates**: Automatically rename duplicate files
- **Clean up**: Remove empty directories after organization
- **Safe**: Ignores project files and important directories

## Installation

### Prerequisites

- Rust and Cargo installed ([rustup.rs](https://rustup.rs/))

### Build from source

```bash
git clone https://github.com/Colgate13/photorec-file-organizer.git
cd photorec-file-organizer
cargo build --release
```

The binary will be available at `target/release/photorec-organizer`

## Usage

Run the program in the directory containing your PhotoRec recovered files:

```bash
cargo run --release
```

Or use the compiled binary:

```bash
./target/release/photorec-organizer
```

### Options

1. **Remove files smaller than 20KB** - Clean up small/corrupted files
2. **Organize files by extension** - Sort files into folders (png, jpg, mp4, etc.)
3. **Both** - Remove small files first, then organize

## Supported File Types

The tool organizes files into folders based on these extensions:

- Images: `png`, `jpg`, `jpeg`, `gif`
- Videos: `mov`, `mp4`, `mkv`
- Audio: `mp3`
- Archives: `zip`
- Others: All other files go into an `others` folder

## Safety

The tool is designed to be safe and will:

- Skip project directories (`target`, `src`, `.git`, `node_modules`)
- Preserve `Cargo.toml` and `Cargo.lock` files
- Only work on the current directory and its subdirectories

## License

MIT

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.
