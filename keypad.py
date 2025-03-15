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
