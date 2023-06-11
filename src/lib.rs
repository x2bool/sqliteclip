use sqlite_loadable::prelude::*;
use sqlite_loadable::api as sqlite_api;
use sqlite_loadable::Result as SQLiteResult;
use sqlite_loadable::Error as SQLiteError;
use sqlite_loadable::define_scalar_function;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

fn clipboard_copy(context: *mut sqlite3_context, values: &[*mut sqlite3_value]) -> SQLiteResult<()> {
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    let content = sqlite_api::value_text(&values[0])?;
    match clipboard.set_contents(content.to_string()) {
        Ok(()) => {
            sqlite_api::result_text(context, content)?;
            Ok(())
        },
        Err(e) => {
            Err(SQLiteError::new_message(&e.to_string()))
        }
    }
}

fn clipboard_paste(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> SQLiteResult<()> {
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    match clipboard.get_contents() {
        Ok(content) => {
            sqlite_api::result_text(context, content)?;
            Ok(())
        },
        Err(e) => {
            Err(SQLiteError::new_message(&e.to_string()))
        }
    }
}

#[sqlite_entrypoint]
pub fn sqlite3_sqliteclip_init(db: *mut sqlite3) -> SQLiteResult<()> {
    define_scalar_function(db, "clipboard_copy", 1, clipboard_copy, FunctionFlags::UTF8)?;
    define_scalar_function(db, "clip_copy", 1, clipboard_copy, FunctionFlags::UTF8)?;
    define_scalar_function(db, "clipboard_paste", 0, clipboard_paste, FunctionFlags::UTF8)?;
    define_scalar_function(db, "clip_paste", 0, clipboard_paste, FunctionFlags::UTF8)?;

    Ok(())
}
