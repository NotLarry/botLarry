#!/usr/bin/python3

import time
import sys
import Adafruit_GPIO.SPI as SPI
import Adafruit_SSD1306
import subprocess

from PIL import Image
from PIL import ImageDraw
from PIL import ImageFont
from datetime import datetime

# Raspberry Pi pin configuration:
RST = None     # on the PiOLED this pin isnt used
# Note the following are only used with SPI:
DC = 23
SPI_PORT = 0
SPI_DEVICE = 0

toggle = 1

try:
    progName = (sys.argv[0])
    screenSide = (sys.argv[1])
    displayText = (sys.argv[2])
except IndexError:
    print( 'Usage:', progName, ' left/right "text"' )
    sys.exit(1)

if screenSide == 'left':
    disp = Adafruit_SSD1306.SSD1306_128_64(rst=RST, i2c_address=0x3D) # left
elif screenSide == 'right':
    disp = Adafruit_SSD1306.SSD1306_128_64(rst=RST, i2c_address=0x3C) # right
else :
    raise SystemExit

# Initialize library
disp.begin()

# Clear display
disp.clear()
disp.display()

width = disp.width
height = disp.height
image = Image.new('1', (width, height))
x = 0
padding = -2
top = padding

while True:
    toggle = (toggle + 5) % 40 

# Get drawing object to draw on image

    draw = ImageDraw.Draw(image)
    
# Draw a black filled box to clear the image.
    draw.rectangle((0,0,width,height), outline=0, fill=0)

# Draw text
# Load default font
# load time
    now = datetime.now()
    inTime = now.strftime("%H:%M:%S")

#    font = ImageFont.load_default()
#    font = ImageFont.truetype('m20.TTF', 30)
    font = ImageFont.truetype('dfhei1b-gb.TTF', 10)
    draw.text((x, top+toggle), inTime, font=font, fill=1)

    disp.image(image)
    disp.display()
 #   time.sleep(.5)
