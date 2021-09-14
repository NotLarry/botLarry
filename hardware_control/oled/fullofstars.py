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



# Define display dimensions 
WIDTH = 128
HEIGHT = 64
leftADDR = 0x3d
rightADDR = 0x3c
rst = DigitalInOut(board.D7)
displeft = adafruit_ssd1306.SSD1306_I2C(WIDTH, HEIGHT, i2c, addr=leftADDR, reset=rst)
dispright = adafruit_ssd1306.SSD1306_I2C(WIDTH, HEIGHT, i2c, addr=rightADDR, reset=rst)
# Create the digital out used for display reset

# Loop forever drawing random pixels

while True:

    display = random.choice([displeft, dispright])
    display.fill(0)
    display.show()
    for _ in range(42):
 
    # Create the display

        x = random.randrange(WIDTH)
        y = random.randrange(HEIGHT)
        display.pixel(x, y, 1)
    display.show()
    time.sleep(0.5)
    display.fill(0)
