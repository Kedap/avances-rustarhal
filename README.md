Proyecto sin hal

# Ejecutar

1. Tener una instalación nightly
2. Compilar con `cargo build -Z build-std=core --target avr-atmega328p.json --release`
3. Minimizar el binario con `avr-objcopy -O ihex -R .eeprom target/avr-atmega328p/release/rust_ardu.elf output.hex`
4. Subir el código `avrdude -p atmega328p -c arduino -P /dev/ttyUSB0 -U flash:w:output.hex`
