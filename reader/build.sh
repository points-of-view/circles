#!/bin/sh
echo "Building reader app"

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE}" )" &> /dev/null && pwd )

mkdir -p $SCRIPT_DIR/dist/META-INF 
echo "Main-Class: reader.PrintRFIDReader.PrintRFIDTags" > $SCRIPT_DIR/dist/META-INF/MANIFEST.MF
javac -cp $SCRIPT_DIR/vendor/zebra/lib/Symbol.RFID.API3.jar -d $SCRIPT_DIR/dist $SCRIPT_DIR/PrintRFIDReader/PrintRFIDTags.java
cp -r $SCRIPT_DIR/vendor $SCRIPT_DIR/dist/vendor
cd $SCRIPT_DIR/dist
jar -cmvf META-INF/MANIFEST.MF reader.jar reader/PrintRFIDReader/*.class  >/dev/null
