// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import {default as init, add, greet, decrypt} from "../pkg/wasm_encrypt_rs.js";

await init("../pkg/wasm_encrypt_rs_bg.wasm");

// Set the result onto the body
document.getElementById("text").textContent = `Hello World! addResult: ${add(111, 2)}`;

// aaa("abc");

let form = document.getElementById("form");
// ...然后接管表单的提交事件
form.addEventListener("submit", function (event) {
    event.preventDefault();

    // 我们把这个 FormData 和表单元素绑定在一起。
    let data  = new FormData(form);
    let encrypted = data.get("encrypted");
    console.log("输入: " + encrypted);

    let decrypted = decrypt(encrypted, "password");
    console.log("解密: " + decrypted)
});
