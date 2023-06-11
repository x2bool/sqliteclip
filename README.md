# SQLite extension for working with operating system's clipboard

This extension adds the following functions to SQLite:

|  | Copy | Paste |
|--|------|-------|
| Long name | CLIPBOARD_COPY(text) | CLIPBOARD_PASTE() |
| Short name | CLIP_COPY(text) | CLIP_PASTE() |

### How to build

```bash
cargo build --release
```

This step will produce `libsqliteclip.so` or `libsqliteclip.dylib` or `sqliteclip.dll` depending on your operation system.

### How to use

Assuming you have sqlite3 command line tools installed, and `libsqliteclip` library in your current directory:

```bash
sqlite3 # will open SQLite CLI
> .load libsqliteclip # or "sqliteclip" on Windows
```

This will load `sqliteclip` module, and make `CLIPBOARD_COPY` and `CLIPBOARD_PASTE` functions available.
