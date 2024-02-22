use rusqlite::Connection;
use serde::Serialize;
use std::{
    ffi::{c_char, CStr, CString},
    thread,
};

#[derive(Debug, Serialize)]
struct Conversation {
    first_name: String,
    last_name: String,
}

/// # Safety
#[no_mangle]
pub unsafe extern "C" fn invoke(
    db_file: *const c_char,
    callback: extern "C" fn(*const c_char),
) -> u32 {
    let db_file_str = unsafe { CStr::from_ptr(db_file).to_string_lossy().into_owned() };

    thread::spawn(move || {
        let conn = Connection::open(&db_file_str).expect("Failed to open SQLite database");
        let mut stmt = conn
            .prepare("SELECT * FROM conversation")
            .expect("Failed to prepare select statement");

        let session_list = stmt.query_map([], |row| {
            Ok(Conversation {
                first_name: row.get(1).unwrap(),
                last_name: row.get(2).unwrap(),
            })
        });

        let mut convers = vec![];
        for p in session_list.unwrap() {
            convers.push(p.unwrap());
        }

        // 将 Rust 字符串转换为 C 字符串
        let session_list_cstr = CString::new(serde_json::to_string(&convers).unwrap())
            .expect("Failed to convert session list to C string");

        // 调用回调函数，将会话列表传递给其他语言
        callback(session_list_cstr.as_ptr());
    });

    return 0;
}
