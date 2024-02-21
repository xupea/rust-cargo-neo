const path = require("path");
const ffi = require("ffi-napi");

// Define the argument types and return type of the factorial function
const { test } = ffi.Library("../target/i686-pc-windows-msvc/debug/mylib.dll", {
  test: ["int", ["string", "pointer"]],
});

async function demo() {
  new Promise(resolve => {
    const callbackType = ffi.Callback("void", ["string"], (sessionList) => {
      console.log("Session list:", sessionList.length);
      resolve(sessionList.length);
    });

    const dbFile = path.join(__dirname, "session.db");
    console.log("start");
    const a = test(dbFile, callbackType);
    console.log("a from js ： ", a);
  })
}

demo();
