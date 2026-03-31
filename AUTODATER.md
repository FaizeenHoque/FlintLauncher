# Auto-Update System Setup Guide

## Overview

The Flint Launcher now includes a complete auto-update system that checks for new releases on GitHub and allows users to update directly from within the application.

## Current Implementation

### Architecture
- **Rust Backend** (`src-tauri/src/updater.rs`): 
  - `check_for_updates()` - Fetches latest release from GitHub API
  - `download_and_install_update()` - Downloads and installs the update
  
- **Frontend Component** (`src/lib/components/UpdateNotifier.svelte`):
  - Automatically checks for updates on app startup
  - Shows update dialog with release notes
  - Allows user to download and install or dismiss

### How It Works

1. **CheckForUpdates**
   - Fetches from: `https://api.github.com/repos/{owner}/{repo}/releases/latest`
   - Compares semantic versions (e.g., 1.2.3 vs 1.2.4)
   - Returns version info and Windows installer download URL
   - Skips pre-releases and draft releases

2. **Download & Install**
   - Downloads installer to system temp directory
   - Emits `update-ready` event when complete
   - User must restart launcher to install

3. **UI Integration**
   - Modal dialog shows current vs latest version
   - Displays release notes from GitHub
   - "Update Now" triggers download, "Later" dismisses

## Setup Steps

### Step 1: Configure GitHub Owner & Repo

Edit `src-tauri/src/updater.rs`:

```rust
const GITHUB_OWNER: &str = "your-github-username";
const GITHUB_REPO: &str = "flint-launcher";
```

Replace with your actual GitHub username and repository name.

### Step 2: Update Version Numbers

Your app version is currently defined in two places:

**tauri.conf.json:**
```json
"version": "0.1.0"
```

**src-tauri/Cargo.toml:**
```toml
[package]
version = "0.1.0"
```

Keep these in sync and match your git tags.

### Step 3: Update Component Version Reference

In `src/lib/components/UpdateNotifier.svelte`, update the hardcoded version:

```typescript
const currentVersion = '0.1.0'; // Update this when you change your version
```

Or better yet, import it from your config:

```typescript
// At app startup, set a global version
// then import it here
```

### Step 4: GitHub Release Setup

When you're ready to release:

1. **Create a git tag:**
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

2. **Build the Windows installer:**
   ```bash
   npm run tauri build
   ```
   This creates `src-tauri/target/release/bundle/nsis/flint-launcher_0.1.0_x64-setup.exe`

3. **Create GitHub Release:**
   - Go to: https://github.com/{username}/flint-launcher/releases
   - Click "Create a new release"
   - Tag: `v0.1.0` (matches your git tag)
   - Title: `Flint Launcher v0.1.0`
   - Description: Add release notes and changelog
   - Upload the `.exe` installer from the build folder
   - Uncheck "This is a pre-release"

4. **Release Format:**
   Your installer filename needs to contain either:
   - `x64-setup` (current pattern)
   - `.exe` (fallback pattern)

### Step 5: Version Bumping Process

For each new release, follow this workflow:

1. Update version in `tauri.conf.json` and `Cargo.toml`
2. Update version in `UpdateNotifier.svelte`
3. Rebuild the app: `npm run tauri build`
4. Create git tag and GitHub release with the installer

## Testing

### Test Update Check (Local)

1. Create a test release on GitHub with a newer version
2. Change `currentVersion` in UpdateNotifier.svelte to an older version
3. Run the dev build and check if the update dialog appears

### Test Download

1. Click "Update Now"
2. Wait for download to complete
3. Check system temp folder for the installer

## Build Output

When you run `npm run tauri build`, the Windows installer is created at:
```
src-tauri/target/release/bundle/nsis/flint-launcher-{version}_x64-setup.exe
```

## GitHub Actions (Optional)

For automated release builds, you can set up a GitHub Actions workflow:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - uses: pnpm/action-setup@v2
      - uses: actions/setup-node@v3
      - run: pnpm install
      - run: npm run tauri build
      - uses: softprops/action-gh-release@v1
        with:
          files: src-tauri/target/release/bundle/nsis/*.exe
```

## Troubleshooting

### Update not showing
- Check that your version in the code is lower than the GitHub release version
- Ensure GitHub release is published (not draft/pre-release)
- Check browser console for API errors

### Download fails
- Verify GitHub API is accessible from user's network
- Check that the installer filename matches the pattern
- Review the network error message in console logs

## Future Enhancements

- [ ] Automatic version reading from config/build
- [ ] Update progress bar for downloads
- [ ] Background update checking (not just on app start)
- [ ] Signature verification for installer (requires code signing)
- [ ] Changelog display from GitHub format
- [ ] Update installation without manual restart
