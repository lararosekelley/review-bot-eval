//! Open a file in the user's editor, the way `git config --edit` does: honor
//! `$VISUAL` then `$EDITOR`, falling back to a platform default. The editor
//! string may carry flags (e.g. `code -w`), so it's split on whitespace.

use std::path::Path;
use std::process::Command;

use anyhow::{bail, Context, Result};

/// Launch the resolved editor on `path` and wait for it to exit.
pub fn open(path: &Path) -> Result<()> {
    let editor = std::env::var("VISUAL")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| {
            std::env::var("EDITOR")
                .ok()
                .filter(|s| !s.trim().is_empty())
        })
        .unwrap_or_else(|| default_editor().to_string());

    // Through the shell, so an editor string with flags (`code -w`) runs as typed.
    let status = Command::new("sh")
        .arg("-c")
        .arg(format!("{editor} {}", path.display()))
        .status()
        .with_context(|| format!("launching editor `{editor}`"))?;
    if !status.success() {
        bail!("editor `{editor}` exited with {status}");
    }
    Ok(())
}

#[cfg(windows)]
fn default_editor() -> &'static str {
    "notepad"
}

#[cfg(not(windows))]
fn default_editor() -> &'static str {
    "vi"
}
