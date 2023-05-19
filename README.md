# rust_fractals
This is a set of implemented fractals implemented in Rust using web assembly.

**Building**
cargo build
wasm-pack build
cd www
npm run start

**Notice**
If you get Error: error:0308010C:digital envelope routines::unsupported starting the application add the following node option
export NODE_OPTIONS=--openssl-legacy-provider
