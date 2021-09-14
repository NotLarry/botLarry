# Import the needed libraries
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
# rst = DigitalInOut(board.D7)
RST = None

# Create the display
display = Adafruit_SSD1306.SSD1306_128_64(rst=RST, i2c_address=0x3D) # left
# display = Adafruit_SSD1306.SSD1306_I2C(WIDTH, HEIGHT, i2c, addr=ADDR, reset=rst)

# Initialize library.
display.begin()

# Clear display.
display.clear()
display.display()


# Define pixel location
x = 42
y = 23
# Draw the pixel
display.pixel(x, y, 1)
display.show()
