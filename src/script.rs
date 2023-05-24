use std::error::Error;
use std::process::{Command, Stdio};

pub(crate) fn convert_path(path: String) -> String {
    let mut p = path.replace('/', ":");
    if p.starts_with(':') {
        p = format!("Macintosh HD{p}");
    }

    p
}

pub(crate) fn run_script(path: String, num: i32) -> Result<(), Box<dyn Error>> {
    let cmd = spawn_cmd(&path, num);

    let output = Command::new("osascript")
        .arg("-e")
        .arg(cmd)
        .stderr(Stdio::piped())
        .output()?;

    if !output.status.success() {
        Err(String::from_utf8_lossy(&output.stderr))?;
    }

    Ok(())
}

fn spawn_cmd(path: &str, num: i32) -> String {
    format!(
        r#"
        tell application "Finder"
            open file "{path}" using alias "Macintosh HD:Applications:Preview.app"
        end tell

        tell application "System Events"
            tell process "Preview"
                set frontmost to true
                tell menu bar 1
                    tell menu "go"
                        click
                        tell menu item "Go to Page…"
                            click
                        end tell
                        end tell
                    end tell
                    tell window 1
                        tell sheet 1
                        tell text field 1
                            keystroke {num}
                        end tell
                        tell button "OK"
                            click
                        end tell
                    end tell
                end tell
            end tell
        end tell
        "#
    )
}
