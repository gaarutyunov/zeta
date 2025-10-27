# Zeta - Voice Zettelkasten

A modern, voice-first Zettelkasten note-taking application built with Dioxus and powered by Claude AI for intelligent voice transcription.

## Features

- **Voice-First Interface**: Create notes using your voice with Claude AI-powered transcription
- **Zettelkasten Method**: Organize your thoughts with linked, atomic notes
- **Smart Tagging**: Automatic tag generation from voice notes
- **Cross-Platform**: Works as a web app or desktop application
- **Local-First**: Your notes are stored locally with SurrealDB
- **Beautiful UI**: Modern, dark-themed interface

## Prerequisites

- Rust (1.70 or later)
- An Anthropic API key ([Get one here](https://console.anthropic.com/))

## Installation

### For Web (GitHub Pages)

The app is automatically deployed to GitHub Pages. Simply visit the deployed URL and enter your Anthropic API key.

### For Desktop

1. Clone the repository:
```bash
git clone https://github.com/yourusername/zeta.git
cd zeta
```

2. Build and run:
```bash
cargo run --features desktop
```

## Development

### Running in Development Mode

For web development:
```bash
dx serve --features web
```

For desktop development:
```bash
cargo run --features desktop
```

### Building for Production

Web build:
```bash
dx build --release --features web
```

Desktop build:
```bash
cargo build --release --features desktop
```

## Configuration

The application requires an Anthropic API key to function. You can provide it in two ways:

1. **Via UI**: Enter your API key in the application when prompted
2. **Via Environment Variable**: Set `ANTHROPIC_API_KEY` before running

```bash
export ANTHROPIC_API_KEY=your-api-key-here
```

## Architecture

- **Frontend**: Dioxus (Rust web framework)
- **AI**: Claude API for voice transcription and note generation
- **Database**: SurrealDB (native) or in-memory storage (web)
- **Audio**: Web Audio API for browser-based recording

## Project Structure

```
zeta/
├── src/
│   ├── main.rs       # Main application and UI components
│   ├── models.rs     # Data models for notes
│   ├── db.rs         # Database operations
│   └── api.rs        # Claude API integration
├── assets/
│   └── style.css     # Application styles
├── Cargo.toml        # Rust dependencies
├── Dioxus.toml       # Dioxus configuration
└── README.md
```

## How It Works

1. **Record**: Click "New Voice Note" and record your thoughts
2. **Transcribe**: Claude AI transcribes and structures your note
3. **Organize**: Notes are automatically tagged and can be linked
4. **Search**: Find notes by title, content, or tags

## Technologies Used

- [Dioxus](https://dioxuslabs.com/) - Rust UI framework
- [SurrealDB](https://surrealdb.com/) - Multi-model database
- [Claude AI](https://www.anthropic.com/claude) - Advanced language model
- Web Audio API - Browser audio recording

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - feel free to use this project for any purpose.

## Acknowledgments

- Built with [Dioxus](https://dioxuslabs.com/)
- Powered by [Claude AI](https://www.anthropic.com/claude)
- Database by [SurrealDB](https://surrealdb.com/)
