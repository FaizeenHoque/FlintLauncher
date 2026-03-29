# Java Runtime Bundling

## Overview

Flint Launcher needs Java to run Minecraft. Java can be handled in two ways:

1. **Auto-Download (Default)**: Java is downloaded on first app launch (~1-2GB)
2. **Pre-Bundled (Recommended for Distribution)**: Java is included in the installer

## Pre-Bundling Java

To include Java in your MSI/EXE installer:

### Option 1: npm Script
```bash
npm run bundle-java
npm run tauri build
```

### Option 2: Automated Build Script
Windows:
```bash
build-with-java.bat
```

Linux/macOS:
```bash
chmod +x build-with-java.sh
./build-with-java.sh
```

### Option 3: Combined Build
```bash
npm run build:release
```

## How It Works

### Build Time
1. `bundle-java.js` downloads Java components from Mojang servers
2. Downloads are stored in `src-tauri/resources/java-runtime/`
3. Tauri bundles these files into the installer package
4. Installer size increases by ~1-2GB

### Runtime
1. App detects if Java is bundled
2. If bundled Java exists, copies it to `%APPDATA%/.flint/runtime/`
3. If not bundled, downloads Java from internet on first launch
4. All subsequent launches use the installed Java runtime

## Windows Manifest Components

The following Java runtimes are bundled:
- **jre-legacy** - For older Minecraft versions
- **java-runtime-alpha** - For newer versions (Java 16+)
- **java-runtime-gamma** - Alternative runtime

## Disabling Bundling

Create an empty file named `.skip-java-bundle` in `src-tauri/` to skip Java bundling during the build:

```bash
touch src-tauri/.skip-java-bundle
```

Java will still be downloaded automatically on first app launch.

## Rebuilding Java Bundle

To refresh/update bundled Java files:

```bash
rm -rf src-tauri/resources/java-runtime/*  # Clear existing files
npm run bundle-java  # Download fresh copies
npm run tauri build
```

## Troubleshooting

### "No Java component found" error
- Check internet connection
- Mojang servers may be temporarily down
- Bundling is optional - app will download Java on launch

### Download is very slow
- Parallel downloads are limited to reduce server load
- Initial download can take 15-30 minutes
- Subsequent rebuilds are faster (only downloads missing files)

### Bundle is too large
- Consider distributing without pre-bundled Java
- Users can still install locally without internet after first download
- Or use compression/split installers if needed

## Manual Download

To manually download and check file sizes:

```javascript
// This shows what the bundle command does
const JAVA_MANIFEST_URL = 'https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json';
```
