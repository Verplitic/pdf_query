# PDF Query
A simple Rust-based program that allows macOS users to open PDF documents and jump to specific pages via Terminal. You can also add aliases and bookmarks to documents so you don't need to type the file path and page number each time.

Refer to `cli.rs` or run `pdf_query --help` with compiled binary to see detailed usage. All aliases and bookmarks are stored at `~/.pqconfig` in YAML format.

## Permissions
The program calls `osascript` to execute a piece of AppleScript in order to open documents and jump to pages. You may need to grant *your terminal where you run this program* (NOT the program itself) with Accessibility and Automation permissions. **First run is expected to fail due to lack of permission.**