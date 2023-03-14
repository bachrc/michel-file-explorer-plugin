cargo build --release
rm ./file-plugin.wasm
wasm-tools component new ./target/wasm32-wasi/release/file_explorer_plugin_component.wasm -o ./file-plugin.wasm --adapt ./wasi_snapshot_preview1.wasm
