import time
import Adafruit_SSD1306
from PIL import Image
from PIL import ImageDraw
from PIL import ImageFont
import subprocess

# Raspberry Pi pin configuration:
RST = None     # on the PiOLED this pin isnt used
# Note the following are only used with SPI:
DC = 23
# initialize toggle
toggle = 1 
# define left and right screens
#def set_screen(handed):
#    global disp
#
#    if handed == '0':
#        # set disp to left screen
#        disp = Adafruit_SSD1306.SSD1306_128_64(rst=RST, i2c_address=0x3C)
#    elif handed == '1':
#        # set disp to right screen
#        disp = Adafruit_SSD1306.SSD1306_128_64(rst=RST, i2c_address=0x3D)


while True:

    define screenish(handed):
        if handed == '0':


            disp = Adafruit_SSD1306.SSD1306_128_64(rst=RST, i2c_address=0x3D)
   # set_screen('toggle')

    
    # toggle the value for left/right 
            toggle = (toggle + 1) % 2


# Initialize library.

            disp.begin()

    # Clear display.
            disp.clear()
            disp.display()

# Create blank image for drawing.
# Make sure to create image with mode '1' for 1-bit color.
            width = disp.width
            height = disp.height
            image = Image.new('1', (width, height))

# Get drawing object to draw on image.
            draw = ImageDraw.Draw(image)

# Draw a black filled box to clear the image.
            draw.rectangle((0,0,width,height), outline=0, fill=0)

# Draw some shapes.
# First define some constants to allow easy resizing of shapes.
            padding = -2
            top = padding
            bottom = height-padding
# Move left to right keeping track of the current x position for drawing shapes.
            x = 0


# Load default font.
            font = ImageFont.load_default()

# Alternatively load a TTF font.  Make sure the .ttf font file is in the same directory as the python script!
# Some other nice fonts to try: http://www.dafont.com/bitmap.php
# font = ImageFont.truetype('Minecraftia.ttf', 8)

    # Draw a black filled box to clear the image.
            draw.rectangle((0,0,width,height), outline=0, fill=0)

#    set_screen(toggle)

    # Shell scripts for system monitoring from here : https://unix.stackexchange.com/questions/119126/command-to-display-memory-usage-disk-usage-and-cpu-load
            cmd = "hostname -I | cut -d\' \' -f1"
            IP = subprocess.check_output(cmd, shell = True )
            cmd = "top -bn1 | grep load | awk '{printf \"CPU Load: %.2f\", $(NF-2)}'"
            CPU = subprocess.check_output(cmd, shell = True )
            cmd = "free -m | awk 'NR==2{printf \"Mem: %s/%sMB %.2f%%\", $3,$2,$3*100/$2 }'"
            MemUsage = subprocess.check_output(cmd, shell = True )
            cmd = "df -h | awk '$NF==\"/\"{printf \"Disk: %d/%dGB %s\", $3,$2,$5}'"
            Disk = subprocess.check_output(cmd, shell = True )
    
    # Write two lines of text.
            draw.text((x, top),       "IP: " + str(IP),  font=font, fill=255)
            draw.text((x, top+8),     str(CPU), font=font, fill=255)
            draw.text((x, top+16),    str(MemUsage),  font=font, fill=255)
            draw.text((x, top+25),    str(Disk),  font=font, fill=255)
            draw.text((x, top+33),    str(toggle),  font=font, fill=255)
            draw.text((x, top+41),    str(disp),  font=font, fill=255)

    # Display image.
            disp.image(image)
            disp.display()
            time.sleep(.1)
