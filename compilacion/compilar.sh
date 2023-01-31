cd ..
echo "------------------Comenzado proceso de compilación"
cargo build --release
cp target/release/base_egui compilacion/base_egui_linux_x86
echo "-----------------Compilado para Linux"
cargo build --target x86_64-pc-windows-gnu --release
cp target/x86_64-pc-windows-gnu/release/base_egui.exe compilacion/base_egui_windows_x86.exe
echo "-----------------Compilado para Windows"
trunk build --release
cp -r dist/ compilacion/version_web/
echo "-----------------Compilado para web. Proceso finalizado."
