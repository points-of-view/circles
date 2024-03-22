# Interacting with the reader through `PrintRFIDTags.jar`

## Setup

### With Nix devshell

Simply run `reader:start` to run the reader

### Without nix

- Make sure you have java installed
- Set ENV var `LD_LIBRARY_PATH` to `reader/vendor/zebra/lib/x86_64`
- Start with the following command:
  ```
    java -Djava.library.path="reader/vendor/zebra/lib/x86_64" \
         -cp reader/vendor/zebra/lib/Symbol.RFID.API3.jar \
         reader/PrintRFIDReader/PrintRFIDTags.java
  ```

## Format of output

Once the instance is started correctly it returns all the read tags in the following pattern, with a newline on the end of every tag:

 `key|antennaId|peakRSSI`

- `key`: `string` = the unique EPC associated with every RFID tag
- `antennaId`: `integer` = the ID of the antenna (integer between 1 and 8 on the FX9600)
- `peakRSSI`: `signed` integer = relative read strength (integer between -30 (strong signal) and -80 (weak signal))