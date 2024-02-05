use rusqlite::Connection;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn get_latest_sessions_from_local(
    db_file: *const c_char,
    callback: extern "C" fn(*const c_char),
) {
    let db_file_str = unsafe { CStr::from_ptr(db_file).to_string_lossy().into_owned() };
    // 从本地 SQLite 数据库读取会话列表
    let conn = Connection::open(&db_file_str).expect("Failed to open SQLite database");
    let mut stmt = conn
        .prepare("SELECT * FROM conversation ORDER BY id DESC LIMIT 1")
        .expect("Failed to prepare select statement");
    let session_list: String = stmt.query_row([], |row| row.get(0)).unwrap_or_default();

    // 将 Rust 字符串转换为 C 字符串
    let session_list_cstr =
        CString::new(session_list).expect("Failed to convert session list to C string");

    // 调用回调函数，将会话列表传递给其他语言
    callback(session_list_cstr.as_ptr());
}
