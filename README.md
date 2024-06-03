# zpr24L

## uruchomienie aplikacji
aplikacje mozna uruchomic wykonujac skrypt build.sh
```bash
./build.sh
```

lub recznie wykonujac te czynnosci:

### stworzenie bazy danych
```bash
cd ./database
sudo docker-compose up -d
```

### uruchomienie czesci aplikacji odpowiedzialnej za wyswietlnaie grafow
```bash
cd ../wasm-binman
wasm-pack build --target web
python3 -m http.server 8000 &
```

### uruchomienie glownej czesci aplikacji
```bash
cd ../zpr24l
cargo build
cargo run
```

## usuniecie danych
W celu wyczyszczenia danych aplikacji nalezy w katalogu glownym projektu wykonac polecenia
```bash
rm -rf ./database/data
docker-compose down
```

## budowanie dokumentacji
W katalogu zpr24l lub wasm-binman
```bash
cargo doc --no-deps #jesli nie chcemy generowac dokumentacji zaciaganych bibliotek
cargo doc # jesli chcemy pelna dokumentacje
```
Nastepnie dokumentacja mozna obejrzec w przegladarce uruchamiajac w katalogu ./zpr24l/target/doc/ lub ./wasm-binman/target/doc serwer http korzystajac z komendy
```bash
python3 -m http.server 12345
```
Dokumentacja bedzie dostepna pod adresem:
[http://localhost:12345/zpr24l/](http://localhost:12345/zpr24l/)

## formater kodu
Uzyty zostal formater rustfmt.
instalacja:
```bash
sudo apt install rustfmt
```
Nastepnie w katalogu zpr24l projektu:
```bash
rustfmt --edition 2021 src/*.rs
```

## linting

Aby uruchomic lint'owanie nalezy w katalogu zpr24l wykonac komende:
```bash
cargo check
```

## testy
Testy uruchamiane se podczas buildu projektu przy uzyciu komendy:
```bash
wasm-pack test --node
cargo test
```
Testy mozna uruchomic rowniez manualnie.