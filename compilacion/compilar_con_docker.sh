#!/bin/bash

docker build . -t ubuntu-xenial-rust/compilador-multiplataforma
cd ..
ruta_base_egui=$(pwd)
cd compilacion
docker run --rm -v "$ruta_base_egui":/base_egui ubuntu-xenial-rust/compilador-multiplataforma
