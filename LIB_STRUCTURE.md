# Library Module Structure

This document describes the refactored `lib.rs` file which has been divided into logical, maintainable modules.

## Module Overview

The original monolithic `lib.rs` file has been reorganized into the following modular structure:

```
src-tauri/src/
├── lib.rs              # Main application setup and command registration
├── accounts/
│   └── mod.rs          # Account management commands
├── launchprocess/
│   ├── mod.rs          # Main launch orchestration
│   ├── accounts.rs     # Account retrieval for launching
│   ├── classpath.rs    # Classpath building
│   ├── java.rs         # Java executable discovery
│   ├── launch.rs       # Process spawning
│   ├── paths.rs        # Directory management
│   └── process.rs      # Process detection
└── library/
    └── ... (version management, profiles, etc.)
```

## Module Descriptions

### `lib.rs` - Application Setup
**Lines before:** ~250 | **Lines after:** ~40
**Responsibility:** Clean application entry point

- Module declarations and imports
- Tauri command registration
- Plugin configuration
- Application entry point (`run()` function)

### `accounts/mod.rs` - Account Management (NEW)
**Responsibility:** All account-related Tauri commands

**Functions:**
- `accountcreate()` - Create new player account (max 6 accounts)
- `accountget()` - List all accounts
- `accountgetcurrent()` - Get currently selected account
- `accountsetcurrent()` - Set the active account
- `accountdelete()` - Remove an account (cannot delete current)

**Internal Helpers:**
- `accounts_file_path()` - Location of accounts.json
- `load_accounts_data()` - Load and migrate account format
- `save_accounts_data()` - Persist account data

### `launchprocess/` - Game Launching
**Responsibility:** Launch Minecraft with all configurations

7 focused submodules (see [LAUNCHPROCESS_STRUCTURE.md](LAUNCHPROCESS_STRUCTURE.md) for details)

### `library/` - Version & Profile Management
**Responsibility:** Download/manage versions, profiles, mods

(Existing module, unchanged)

## Benefits of This Refactoring

1. **Smaller Main File** - lib.rs reduced from ~250 to ~40 lines
2. **Focused Modules** - Each module has a single responsibility
3. **Easier Updates** - Account changes go only in accounts/mod.rs
4. **Better Organization** - Clear separation of concerns
5. **Maintainability** - No duplicated data loading logic

## File Structure Comparison

### Before
```
lib.rs (250 lines)
├── Module imports
├── accounts_file_path()
├── load_accounts_data() [duplicated logic]
├── accountcreate()
├── accountget()
├── accountgetcurrent()
├── accountsetcurrent()
├── accountdelete()
└── run()
```

### After
```
lib.rs (40 lines)
├── Module imports
├── run()

accounts/mod.rs (140 lines)
├── accounts_file_path()
├── load_accounts_data() [shared]
├── save_accounts_data() [shared]
├── accountcreate()
├── accountget()
├── accountgetcurrent()
├── accountsetcurrent()
└── accountdelete()
```

## Code Quality Improvements

✅ **Reduced Duplication** - Shared logic (load/save) implemented once
✅ **Better Abstraction** - Internal helpers private to accounts module
✅ **Clearer Imports** - lib.rs imports show exactly what commands are exposed
✅ **Easier Testing** - Individual modules can be tested independently
✅ **Maintainability** - Related functions grouped in one module

## Usage

The public API remains unchanged. All Tauri commands work exactly the same:

```typescript
// Frontend still calls the same commands
await invoke('accountcreate', { username: 'Player1' })
await invoke('accountget')
```

The module reorganization is completely transparent to the application.
