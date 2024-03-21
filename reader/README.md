# Interacting with the reader through `PrintRFIDTags.jar`

`PrintRFIDTags.jar` can be run through `java -jar PrintRFIDTags.jar`

Once the instance is started correctly it returns all the read tags in the following pattern, with a newline on the end of every tag:

 `key|antennaId|peakRSSI`

- `key`: `string` = the unique EPC associated with every RFID tag
- `antennaId`: `integer` = the ID of the antenna (integer between 1 and 8 on the FX9600)
- `peakRSSI`: `signed` integer = relative read strength (integer between -30 (strong signal) and -80 (weak signal))