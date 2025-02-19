# epcmanger

## Desciption

`epcmanager` is to change the EPC data of the RFID to other format.

Change ASCII String to Hex String.
Hex String to ASCII String.

![screen shot](images/screenshot.png)

- EPC size: 64bit, 96bit, 128bit
- Encoder: 8Bit(Normal), 7Bit(Packed), 6Bit(AlphaNumeric)

## build

This application is built by RUST.

### dependencies

this application uses following crates.

- iced
- regex

### build

please use this command.

```sh

cargo run --release

```

## LICENCE

this application is licensed under Apache License Version 2.

