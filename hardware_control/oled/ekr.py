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


def randomcords():
        x_go = random.randrange(WIDTH)
        y_go = random.randrange(HEIGHT)
        d = random.randrange(DIAMETER)
        r = d/2
        while ((x_go + r > 127) or (x_go - r < 1) or (y_go + r > 63) or (y_go - r < 1)):
            print(x_go,y_go,d)
            x_go = random.randrange(WIDTH)
            y_go = random.randrange(HEIGHT)
            d = random.randrange(DIAMETER)
            r = d/2
        return(x_go,y_go,d)

# Define display dimensions 
WIDTH = 128
HEIGHT = 64
DIAMETER = 60
leftADDR = 0x3d
rightADDR = 0x3c
rst = DigitalInOut(board.D7)
displeft = adafruit_ssd1306.SSD1306_I2C(WIDTH, HEIGHT, i2c, addr=leftADDR, reset=rst)
dispright = adafruit_ssd1306.SSD1306_I2C(WIDTH, HEIGHT, i2c, addr=rightADDR, reset=rst)
# Create the digital out used for display reset

# Loop forever drawing random pixels

while True:

#    display = random.choice([displeft, dispright])
#    display.fill(0)
    displeft.fill(0)
    dispright.fill(0)
#    display.show()
    displeft.show()
    dispright.show()
    for _ in range(1):
 
    # Create the display

        x_go, y_go, d =randomcords()

#        display.pixel(x, y, 1)
#        display.circle(x,y,d,color)
        displeft.circle(x_go,y_go,d,1)
        dispright.circle(x_go,y_go,d,1)
        d -= 5
        displeft.show()
        dispright.show()

#    display.show()
    displeft.show()
    dispright.show()
    time.sleep(0.5)
#    display.fill(0)
    displeft.fill(0)
    dispright.fill(0)



