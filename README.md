# Zeta - Voice Zettelkasten

A modern, voice-first Zettelkasten note-taking application built with Dioxus and powered by multiple AI transcription services.

## Features

- **Multiple Transcription Providers**: Choose from Claude AI, OpenAI Whisper, Google Cloud Speech-to-Text, or local Whisper (WebGPU)
- **Voice-First Interface**: Create notes using your voice with AI-powered transcription
- **Zettelkasten Method**: Organize your thoughts with linked, atomic notes
- **Smart Tagging**: Automatic tag generation from voice notes
- **Cross-Platform**: Works as a web app or desktop application
- **Local-First**: Your notes are stored locally with SurrealDB
- **Beautiful UI**: Modern, dark-themed interface
- **Privacy Options**: Use local Whisper for offline, private transcription

## Transcription Providers

Zeta supports multiple voice transcription services:

### Claude AI (Recommended)
- Best for structured notes with auto-generated titles and tags
- Excellent at understanding context and creating meaningful summaries
- Requires: [Anthropic API key](https://console.anthropic.com/)

### OpenAI Whisper
- High-quality transcription with GPT-powered note structuring
- Great accuracy across multiple languages
- Requires: [OpenAI API key](https://platform.openai.com/api-keys)

### Google Cloud Speech-to-Text
- Fast and accurate transcription
- Good for quick voice notes
- Requires: [Google Cloud API key](https://console.cloud.google.com/apis/credentials)

### Local Whisper (WebGPU)  🚧 Experimental
- Privacy-first: runs entirely in your browser
- No API key required
- Requires WebGPU-enabled browser (Chrome/Edge)
- Note: Currently in development

## Prerequisites

- Rust (1.70 or later)
- An API key from your chosen transcription provider (except for Local Whisper)

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

### Setting Up Your Transcription Provider

1. **Via Settings UI** (Recommended):
   - Click the settings icon (⚙️) in the top-right corner
   - Select your preferred transcription provider
   - Enter your API key
   - Click "Save Settings"

2. **Via Environment Variables**:
   ```bash
   # For Claude AI
   export ANTHROPIC_API_KEY=sk-ant-api03-...

   # For OpenAI
   export OPENAI_API_KEY=sk-...

   # For Google Cloud
   export GOOGLE_CLOUD_API_KEY=AIza...
   ```

### Switching Providers

You can change your transcription provider at any time through the Settings menu. This allows you to:
- Try different providers to see which works best for you
- Switch to local Whisper for privacy-sensitive notes
- Use different providers for different types of notes

## Architecture

- **Frontend**: Dioxus (Rust web framework)
- **AI**: Multiple transcription providers (Claude, OpenAI, Google, Local Whisper)
- **Database**: SurrealDB (native) or in-memory storage (web)
- **Audio**: Web Audio API for browser-based recording

## Project Structure

```
zeta/
├── src/
│   ├── main.rs              # Main application and UI components
│   ├── models.rs            # Data models for notes
│   ├── db.rs                # Database operations
│   ├── audio.rs             # Audio recording module
│   ├── components/
│   │   ├── mod.rs
│   │   └── settings.rs      # Settings UI component
│   └── transcription/
│       ├── mod.rs           # Transcription service trait
│       ├── claude.rs        # Claude AI integration
│       ├── openai.rs        # OpenAI Whisper integration
│       ├── google.rs        # Google Cloud Speech-to-Text
│       └── whisper_web.rs   # Local Whisper (WebGPU)
├── assets/
│   └── style.css            # Application styles
├── .github/workflows/
│   └── deploy.yml           # GitHub Pages deployment
├── Cargo.toml               # Rust dependencies
├── Dioxus.toml              # Dioxus configuration
└── README.md
```

## How It Works

1. **Configure**: Choose your preferred transcription provider in Settings
2. **Record**: Click "New Voice Note" and record your thoughts
3. **Transcribe**: AI transcribes and structures your note with title and tags
4. **Organize**: Notes are automatically tagged and can be linked
5. **Search**: Find notes by title, content, or tags

## Technologies Used

- [Dioxus](https://dioxuslabs.com/) - Rust UI framework
- [SurrealDB](https://surrealdb.com/) - Multi-model database
- [Claude AI](https://www.anthropic.com/claude) - AI transcription and structuring
- [OpenAI Whisper](https://platform.openai.com/docs/guides/speech-to-text) - Speech-to-text API
- [Google Cloud Speech-to-Text](https://cloud.google.com/speech-to-text) - Cloud transcription service
- [Whisper Web (Transformers.js)](https://github.com/xenova/whisper-web) - Local browser-based transcription
- Web Audio API - Browser audio recording

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - feel free to use this project for any purpose.

## Acknowledgments

- Built with [Dioxus](https://dioxuslabs.com/)
- Powered by [Claude AI](https://www.anthropic.com/claude)
- Database by [SurrealDB](https://surrealdb.com/)
