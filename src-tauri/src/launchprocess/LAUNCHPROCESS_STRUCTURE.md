# Launchprocess Module Structure

This document describes the refactored `launchprocess` module which has been divided into logical, maintainable components.

## Module Overview

The original monolithic `launchprocess.rs` file has been reorganized into the following modular structure:

```
src-tauri/src/launchprocess/
├── mod.rs          # Main module declaration and orchestration
├── accounts.rs     # Account management
├── classpath.rs    # Classpath building from libraries
├── java.rs         # Java executable discovery
├── launch.rs       # Minecraft process spawning
├── paths.rs        # Directory and path management
└── process.rs      # Minecraft process detection
```

## Module Descriptions

### `mod.rs` - Main Orchestration
**Responsibility:** Orchestrates the entire launch flow
- Contains the main `#[tauri::command] launchprocess()` function
- Coordinates all other modules to launch Minecraft
- Handles profile vs. direct version launches
- Provides logging throughout the launch process

### `java.rs` - Java Discovery
**Responsibility:** Finding the Java executable
- `find_java_executable()` - Locates Java in multiple fallback order:
  1. Version metadata (flint_meta.json)
  2. Bundled Java runtimes (gamma, alpha, legacy variants)
  3. System Java from PATH

### `process.rs` - Process Management
**Responsibility:** Detecting running Minecraft instances
- `is_minecraft_running()` - Checks if java.exe is already running
- Prevents multiple Minecraft instances from launching

### `paths.rs` - Directory Management
**Responsibility:** Path and directory structure
- `GameDirectories` struct - Represents the complete directory layout
- `setup_directories()` - Creates profile-specific or vanilla directories
- `emit_log()` - Sends formatted log messages to the frontend

### `accounts.rs` - Account Management
**Responsibility:** Account configuration
- `get_current_account()` - Reads current selected account from accounts.json
- Handles migration from old array format to new object format
- `get_current_account_with_log()` - Version with error logging

### `classpath.rs` - Classpath Assembly
**Responsibility:** Building Java classpath
- `build_classpath()` - Assembles classpath from libraries and main JAR
- `get_asset_index()` - Extracts asset index from version JSON
- `get_main_class()` - Extracts main class from version JSON

### `launch.rs` - Process Spawning
**Responsibility:** Launching the Minecraft process
- `LaunchConfig` struct - Configuration for launching
- `spawn_minecraft_process()` - Spawns Java process with all game parameters

## Benefits of This Structure

1. **Readability** - Each module has a single, clear responsibility
2. **Maintainability** - Changes to one aspect (e.g., Java discovery) are isolated
3. **Testability** - Individual modules can be tested independently
4. **Reusability** - Components like `find_java_executable()` can be used elsewhere
5. **Documentation** - Each module has clear comments explaining its purpose

## File Size Comparison

**Before:** `launchprocess.rs` - ~400 lines (single file)

**After:**
- `mod.rs` - ~130 lines
- `java.rs` - ~40 lines
- `process.rs` - ~15 lines
- `paths.rs` - ~55 lines
- `accounts.rs` - ~35 lines
- `classpath.rs` - ~40 lines
- `launch.rs` - ~75 lines

**Total:** ~390 lines (better organized across focused modules)

## Usage

The public API remains unchanged. The `launchprocess` command is still imported and used exactly the same way in `lib.rs`:

```rust
use launchprocess::launchprocess;
```

All refactoring is internal and transparent to the calling code.
