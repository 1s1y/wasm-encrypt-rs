## Rust WebAssembly 实现的加解密 demo

可以让你的静态博客实现一定程度上的内容保护。

对应的 js 端加解密实现见 [cloudflare worker encrypt](https://github.com/FAE56/cloudflare-worker-encrypt)

通过 `pbkdf2` 导出密钥, 然后 `aes-256-gcm` 加解密。

```shell
# 生成项目脚手架
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project

# 编译 wasm web
wasm-pack build --target web

# 项目引用见 www/index.js
```

参考资料
- https://doc.rust-lang.org/book/
- https://rustwasm.github.io/docs.html
- https://wasmbyexample.dev/home.en-us.html