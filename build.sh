if [ ! -d "binaryen-version_130" ]; then
    curl -L https://github.com/WebAssembly/binaryen/releases/download/version_130/binaryen-version_130-x86_64-linux.tar.gz | tar zx
fi
if [ ! -d "wasm-minimal-protocol" ]; then
    git clone https://github.com/astrale-sharp/wasm-minimal-protocol
fi
cd wasm-minimal-protocol/crates/wasi-stub
cargo install --path .
cd ../../..
rustup target add wasm32-wasip1
cp README.md typst-package/
cp LICENSE typst-package/
cargo build --release --target wasm32-wasip1
./wasm-minimal-protocol/target/release/wasi-stub -r 0 ./target/wasm32-wasip1/release/jq.wasm -o typst-package/jq.wasm
./binaryen-version_130/bin/wasm-opt typst-package/jq.wasm -O3 --enable-bulk-memory -o typst-package/jq.wasm