cargo build --release
wasm-tools component new ./target/wasm32-wasi/release/file_explorer_plugin_component.wasm -o ./file-plugin.wasm --adapt ./wasi_snapshot_preview1.wasm
mv -f ./file-plugin.wasm $HOME/michel/plugins/file-plugin.wasm