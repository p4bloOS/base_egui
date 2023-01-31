# Usamos una imagen antigua de linux (año 2016) porque una versión antigua de glibc (2.23) aporta mayor portabilidad. Rust siempre enlaza dinámicamente dicha biblioteca.
FROM ubuntu:xenial

# Hemos hecho las pruebas con bash, así que lo usaremos para evitar posibles problemas
SHELL ["/bin/bash", "-c"]

# Instalamos algunos paquetes necesarios para que nada falle
RUN apt-get update -y
RUN apt-get install curl -y
#RUN apt-get install git -y
RUN apt-get install cmake -y
RUN apt-get install g++ -y
RUN apt-get install pkg-config -y
RUN apt-get install libfontconfig1-dev -y
RUN apt-get install mingw-w64 -y
RUN apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev -y
RUN apt-get upgrade -y

# Instalamos rust, trunk, y los targets de las plataformas para las cuales queremos compilar.
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown
RUN rustup target add x86_64-pc-windows-gnu

WORKDIR /base_egui/compilacion/
CMD ["sh", "compilar.sh"]
