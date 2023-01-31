#!/bin/bash

docker build . -t ubuntu-xenial-rust/servidor-web
cd ..
ruta_base_egui=$(pwd)
docker run -p 8080:8080 --rm -v "$ruta_base_egui":/base_egui ubuntu-xenial-rust/servidor-web
