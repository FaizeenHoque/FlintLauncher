# Building Flint Launcher with Java Runtime

## Quick Start - Build MSI with Java Bundled

### Windows (Easiest)
```bash
build-with-java.bat
```

Or manually:
```bash
npm run bundle-java
npm run tauri build
```

### macOS/Linux
```bash
chmod +x build-with-java.sh
./build-with-java.sh
```

Or manually:
```bash
npm run bundle-java
npm run tauri build
```

---

## What Happens During Build

### 1. Java Bundling Phase
The `bundle-java.js` script:
- Downloads Java components from Mojang servers
- Stores them in `src-tauri/resources/java-runtime/`
- Takes 5-15 minutes on first run
- Faster on subsequent runs (only downloads missing files)

Downloaded components:
- **jre-legacy** (~350MB) - For Minecraft 1.16 and earlier
- **java-runtime-alpha** (~230MB) - For Minecraft 1.17+
- **java-runtime-gamma** (~220MB) - Alternative runtime

Total size: ~800MB to 1.2GB

### 2. Tauri Build Phase
Tauri:
- Packages the frontend (SvelteKit)
- Includes bundled Java in resources
- Builds MSI installer for Windows
- Final installer size: 1-2GB (includes Minecraft data)

### 3. Installation Phase
When user installs the app:
- MSI extracts app and bundled Java
- On first launch, Java is copied to `%APPDATA%/.flint/runtime/`
- App is ready to use immediately

---

## Build Without Java Bundling

If you want to build without Java (users will download on first launch):

```bash
npm run tauri build
```

Or create `.skip-java-bundle` file:
```bash
touch src-tauri/.skip-java-bundle
npm run tauri build
```

---

## NPM Scripts Reference

| Command | What It Does |
|---------|-------------|
| `npm run dev` | Start dev server |
| `npm run check` | Check TypeScript |
| `npm run bundle-java` | Download Java files |
| `npm run tauri build` | Build MSI without Java |
| `npm run build:release` | Build MSI with Java (recommended) |

---

## Troubleshooting

### Build fails with "Failed to download Java"
- Check internet connection
- Mojang servers may be temporarily unavailable
- Bundling is optional - build without it and Java downloads on app launch

### Java download is very slow
- This is normal - downloading ~1GB takes time
- Subsequent rebuilds only download missing files
- Consider your internet speed and file system

### How to rebuild Java bundle
```bash
# Clear old files
rm -rf src-tauri/resources/java-runtime/*

# Download fresh copies
npm run bundle-java

# Build
npm run tauri build
```

### Can I use pre-downloaded Java?
Yes! Manually place Java files in `src-tauri/resources/java-runtime/jre-legacy/`, etc.
Structure should match: `resources/java-runtime/{component}/bin/java.exe`

---

## Output Files

After building, look for:
- `src-tauri/target/release/bundle/nsis/FlintLauncher_*.exe` - MSI installer stub
- `src-tauri/target/release/bundle/msi/FlintLauncher_*.msi` - Full MSI installer

---

## Notes

- Building with Java takes 20-40 minutes
- Building without Java takes 5-10 minutes
- The bundled Java is essential for offline installations
- Users without Java can still use the app - it downloads on first launch
