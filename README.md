# rust_fractals<br>
This is a set of implemented fractals implemented in Rust using web assembly.<br>
<br>
**Building**
cargo build<br>
wasm-pack build<br>
cd www<br>
npm run start<br>
<br>
**Notice**<br>
If you get Error: error:0308010C:digital envelope routines::unsupported starting the application add the following node option
export NODE_OPTIONS=--openssl-legacy-provider
