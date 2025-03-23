"""
keypad.py may be the heart of botlarry.  It will need to perform a signifigant amount of the process.

1) send dailtone to earpiece. (timeout 20 seconds)
2) when a key is pressed send a tone specific to that key to the earpiece, record the digit
3) When 10 digits have been processed
4) Check if first digit is '1', if so call audio.py with local calls only, then return to dialtone
5) check the first 3 digits for area code.
  a) for 666 area codes look up the 10 digit # in database
    1) if it has an entry call audio.py with coresponding mp3 file
    2) if it does not have an entry call audio.py with 'number is not assigned' mp3
    3) return to hook.py

"""
from matrix_keypad import RPi_GPIO




# the following is keypad code for the nodemcu.  This will need to be replaced.

import adafruit_matrixkeypad
from digitalio import DigitalInOut
import board

# botlarry's 3x4 matrix keypad
cols = [DigitalInOut(x) for x in (board.C2, board.C0, board.C4)]
rows = [DigitalInOut(x) for x in (board.C1, board.C6, board.C5, board.C3)]
keys = ((1, 2, 3),
        (4, 5, 6),
        (7, 8, 9),
        ('*', 0, '#'))

keypad = adafruit_matrixkeypad.Matrix_Keypad(rows, cols, keys)

while True:
    keys = keypad.pressed_keys
    if keys:
        print("Pressed: ", keys)
    time.sleep(0.1)
