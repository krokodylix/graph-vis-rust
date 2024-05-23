#!/bin/bash

# stworzenie bazy danych
cd ./database
sudo docker-compose up -d


# uruchomienie czesci aplikacji odpowiedzialnej za wyswietlnaie grafow
cd ../wasm-binman
wasm-pack build --target web
python3 -m http.server 8000 &

# uruchomienie glownej czesci aplikacji
cd ../zpr24l
cargo build
cargo run