# iOS Build Configuration

This directory contains configuration files for building Zeta as an iOS app.

## Files

- `Info.plist` - iOS app metadata and permissions
- `Zeta.entitlements` - App capabilities and entitlements (Siri for Speech Recognition)

## Prerequisites

- macOS (for iOS development)
- Xcode 14.0 or later
- Rust with iOS targets
- Dioxus CLI

## Setup

### 1. Install Rust iOS Targets

```bash
rustup target add aarch64-apple-ios           # iOS devices (ARM64)
rustup target add aarch64-apple-ios-sim       # iOS Simulator (M1/M2 Macs)
rustup target add x86_64-apple-ios            # iOS Simulator (Intel Macs)
```

### 2. Install Dioxus CLI

```bash
cargo install dioxus-cli
```

## Building

### Build for iOS Device

```bash
# From the project root
dx build --platform ios --release
```

Or manually:
```bash
cargo build --target aarch64-apple-ios --features ios --release
```

### Build for iOS Simulator (M1/M2 Mac)

```bash
cargo build --target aarch64-apple-ios-sim --features ios --release
```

### Build for iOS Simulator (Intel Mac)

```bash
cargo build --target x86_64-apple-ios --features ios --release
```

## Running on Simulator

```bash
dx serve --platform ios
```

## App Capabilities

The iOS app has the following capabilities enabled:

### Speech Recognition
- **Entitlement**: `com.apple.developer.siri`
- **Permission**: `NSSpeechRecognitionUsageDescription`
- Enables on-device speech-to-text using Apple's Speech framework

### Microphone
- **Permission**: `NSMicrophoneUsageDescription`
- Required for recording voice notes

## Privacy

The iOS version uses Apple's Speech framework for on-device transcription:
- **Completely private** - audio never leaves your device
- **No API keys required** - free to use
- **Works offline** for supported languages
- Better battery life compared to cloud services

## Distribution

### TestFlight / App Store

1. **Code Signing**:
   - Configure signing in Xcode or `dx` CLI
   - Requires Apple Developer account ($99/year)

2. **Build for Distribution**:
   ```bash
   dx build --platform ios --release
   ```

3. **Create IPA**:
   - Use Xcode to archive and export
   - Or use `dx` CLI with signing configured

4. **Upload to App Store Connect**:
   ```bash
   # Using Xcode or Transporter app
   ```

### Enterprise Distribution

Follow Apple's enterprise distribution guidelines with proper provisioning profiles.

## Troubleshooting

### Missing Permissions

If speech recognition doesn't work, ensure:
1. `NSSpeechRecognitionUsageDescription` is in Info.plist
2. `NSMicrophoneUsageDescription` is in Info.plist
3. User has granted permissions in Settings

### Siri Entitlement

The Siri entitlement is required for Speech Recognition. If you see errors:
1. Ensure `Zeta.entitlements` is included in your build
2. Check your Apple Developer account has Siri capability
3. Regenerate provisioning profiles if needed

### Build Errors

If you see linking errors:
```bash
# Clean and rebuild
cargo clean
dx build --platform ios --release
```

## Development Tips

1. **Use Simulator for Development**:
   - Faster iteration
   - No code signing required
   - Speech recognition works in simulator

2. **Test on Real Device**:
   - Better performance testing
   - Real microphone testing
   - Actual privacy guarantees

3. **Monitor Console**:
   - Use Xcode console to see Rust logs
   - Check for permission issues
   - Debug speech recognition errors

## Further Reading

- [Dioxus Mobile Guide](https://dioxuslabs.com/learn/0.7/guides/mobile)
- [Apple Speech Framework](https://developer.apple.com/documentation/speech)
- [iOS App Distribution](https://developer.apple.com/documentation/xcode/distributing-your-app-for-beta-testing-and-releases)
