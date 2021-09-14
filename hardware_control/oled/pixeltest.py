

# from PIL import Image
# from PIL import ImageDraw
# from PIL import ImageFont

# import subprocess



# Import the needed libraries
import time
import sys
import random
import board
import busio
from digitalio import DigitalInOut
import Adafruit_SSD1306

# Create the I2C bus
i2c = busio.I2C(board.SCL, board.SDA)

# Define display dimensions and I2C address
WIDTH = 128
HEIGHT = 64
ADDR = 0x3d

# Create the digital out used for display reset
RST = None
# rst = DigitalInOut(board.D7)



try:
    screenSide = (sys.argv[1])
except IndexError:
    raise SystemExit("Usage: left or right")

if screenSide == 'left':
    display = Adafruit_SSD1306.SSD1306_128_64(rst=RST, i2c_address=0x3D) # left
elif screenSide == 'right':
    display = Adafruit_SSD1306.SSD1306_128_64(rst=RST, i2c_address=0x3C) # right
else :
    raise SystemExit("Usage: left or right")

# Create the display
# display = adafruit_ssd1306.SSD1306_I2C(WIDTH, HEIGHT, i2c, addr=ADDR, reset=rst)
display.fill(0)
display.show()

# Loop forever drawing random pixels
while True:
    for _ in range(500):
        x = random.randrange(WIDTH)
        y = random.randrange(HEIGHT)
        display.pixel(x, y, 1)
    display.show()
    time.sleep(0.5)
    display.fill(0)




