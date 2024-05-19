# zpr24L

## uruchomienie aplikacji
W katalogu database nalezy wykonac komende:
```bash
docker-compose up -d
```
Nastepnie w katalogu zpr24l nalezy wykonac komendy:
```bash
cargo build
cargo run
```

W celu wyczyszczenia danych aplikacji nalezy w katalogu glownym projektu wykonac polecenia
```bash
sudo rm -rf ./database/data
docker-compose down
```

## budowanie dokumentacji
W katalogu zpr24l
```bash
cargo docs --no-deps #jesli nie chcemy generowac dokumentacji zaciaganych bibliotek
cargos docs # jesli chcemy pelna dokumentacje
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