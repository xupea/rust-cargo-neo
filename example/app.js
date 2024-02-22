const path = require("path");
const ffi = require("ffi-napi");

const libPath = {
  "win32": "../target/i686-pc-windows-msvc/debug/mylib.dll",
  "darwin": "../target/debug/libmylib.dylib"
}
// Define the argument types and return type of the factorial function
const { invoke } = ffi.Library(libPath[process.platform], {
  invoke: ["int", ["string", "pointer"]],
});

async function demo() {
  new Promise(resolve => {
    const callbackType = ffi.Callback("void", ["string"], (sessionList) => {
      console.log("Session list:", sessionList.length);
      resolve(sessionList.length);
    });

    const dbFile = path.join(__dirname, "session.db");
    console.log("start");
    const a = invoke(dbFile, callbackType);
    console.log("a from js ： ", a);
  })
}

demo();
