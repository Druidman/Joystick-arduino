# Joystick-arduino

This repository enables you to emulate `joystick device` with **NON-HID** `arduino-boards`
## Project consists of:
  - Joystick-driver
  - arduino-device
# How does it work?
Driver and device communicate using `2-way USB communication`. (Beacuse it enables project to grow)\
Driver receives `sensor-data` from arduino-board and emulates `virtual-device` which acts as a FlightStick.

# Setup

# REQUIREMENTS 
  - linux pc
  - arduino-board
  - two potentiometers / linear hall effect sensor for example `SS49E`\
  ! **if you use hall sensors then you also need neodynium magnets** !
  - gimbal ( Can be found in `stl-files` folder )
# How to run it?
  1. Send arduino code (from `arduino-device`)  to the board
  2. Setup .env file
  3. Run main.rs file (from `driver`)

