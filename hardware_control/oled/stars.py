# Import the needed libraries
import time
import sys
import random
import board
import busio
from digitalio import DigitalInOut
import adafruit_ssd1306

# Create the I2C bus
i2c = busio.I2C(board.SCL, board.SDA)

# get screen location and set I2C address
try:
    screenSide = (sys.argv[1])
except IndexError:
    raise SystemExit("Usage: left or right")

if screenSide == 'left':
    ADDR = 0x3d
elif screenSide == 'right':
    ADDR = 0x3c
else :
    raise SystemExit("Usage: left or right")


# Define display dimensions 
WIDTH = 128
HEIGHT = 64

# Create the digital out used for display reset
rst = DigitalInOut(board.D7)

# Create the display
display = adafruit_ssd1306.SSD1306_I2C(WIDTH, HEIGHT, i2c, addr=ADDR, reset=rst)
display.fill(0)
display.show()

# Loop forever drawing random pixels
while True:
    for _ in range(45):
        x = random.randrange(WIDTH)
        y = random.randrange(HEIGHT)
        display.pixel(x, y, 1)
    display.show()
    time.sleep(0.5)
    display.fill(0)
