const path = require("path");
const ffi = require("ffi-napi");

// Define the argument types and return type of the factorial function
const { get_latest_sessions_from_local } = ffi.Library(
  "../target/release/libmylib.dylib",
  {
    factorial: ["uint64", ["uint64"]],
    get_latest_sessions_from_local: ["void", ["string", "pointer"]],
  }
);

// 定义回调函数的类型
const callbackType = ffi.Callback("void", ["string"], (sessionList) => {
  console.log("Session list:", sessionList);
});

for (let i = 0; i < 10000; i++) {
  console.log(`Session list ${i}:`)
  // 调用 C 函数
  const dbFile = path.join(__dirname, "session.db");
  get_latest_sessions_from_local(dbFile, callbackType);
}
