# Joystick-arduino

This repository enables you to emulate `joystick device` with **NON-HID** `arduino-boards`
## Project consists of:
  - Joystick-driver
  - arduino-device
# How does it work?
Driver and device communicate using `2-way USB communication`. (Beacuse it enables project to grow)\
Driver receives `sensor-data` from arduino-board and emulates `virtual-device` which acts as a FlightStick.

