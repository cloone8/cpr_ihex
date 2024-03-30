# Changelog

This file will document the most important changes for each released version, starting from version 0.4.0

## [v1.0.0]

### Features
- Add support for showing the hexfile start address in the main GUI

## [v0.5.1]

### Bugfixes
- Fixed a bug where the GUI would still open even if a CLI command was issued
- Fixed a bug where the program would crash if a binary or hexdump command was issued while the hexfile contained extender segment or linear address records

## [v0.5.0]

### Features
- Better CLI option handling. An invalid combination of options will now show an error immediately, instead of crashing halfway through the program.
- Support for segment and linear extended base addresses for data records

## [v0.4.0]

### Features
- Add CLI command to hexdump the data contents of an input file and exit without showing the GUI
- Add CLI command to binary-dump the data contents of an input file and exit without showing the GUI
