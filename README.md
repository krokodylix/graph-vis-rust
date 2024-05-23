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
sudo rm -rf ./database/data
docker-compose down
```

## budowanie dokumentacji
W katalogu zpr24l
```bash
cargo docs --no-deps #jesli nie chcemy generowac dokumentacji zaciaganych bibliotek
cargo docs # jesli chcemy pelna dokumentacje
```
Nastepnie dokumentacje mozna uruchomic w przegladarce przy uzyciu np python httpservera
```bash
python3 -m http.server
```
Dokumentacja bedzie dostepna pod adresem:
[http://localhost:8000/zpr24l/](http://localhost:8000/zpr24l/)

## formater kodu
Uzyty zostal formater rustfmt.
instalacja:
```bash
sudo apt install rustfmt
```
Nastepnie katalogu zpr24l projektu:
```bash
rustfmt --edition 2021 src/*.rs
```

## linting

Aby uruchomic lint'owanie nalezy w katalogu zpr24l wykonac komende:
```bash
cargo check
```

## testy