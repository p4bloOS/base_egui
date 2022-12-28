IGU multiplataforma hecha en Rust

Usando la plantilla de eframe y la biblioteca egui.

Esta aplicación puede ejecutarse tanto en nativo como en un navegador web.

En realidad, solo son pequeña modificación de la plantilla original, pensando en que este proyecto se use también como plantilla para futuros programas. Se ha reestrucuturado parte del código, añadiendo fuentes tipográficas, modificando la apariencia gráfica, cambiando la interfaz resultante y recomentando el código en español.

Instrucciones de compilación:

    $ cd base_gui/
    $ cargo run --release
 
* Primero deben instalarse las dependencias. En linux (debian):
    $ sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev 

Para compilar a WASM (versión web):

    $ trunk serve # para compilar y arrancar un servidor que escucha en http://127.0.0.1:8080
    $ trunk build --release # para generar los archivos de la aplicación web en el directorio /dist

* Primero debería instalarse trunk con: $cargo install --locked trunk
    y también el target para WASM con: $rustup target add wasm32-unknown-unknown