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

# #####################################################
# Python Library for 3x4 matrix keypad using
# 7 of the avialable GPIO pins on the Raspberry Pi. 
# 
# This could easily be expanded to handle a 4x4 but I 
# don't have one for testing. The KEYPAD constant 
# would need to be updated. Also the setting/checking
# of the colVal part would need to be expanded to 
# handle the extra column.
# 
# Written by Chris Crumpacker
# May 2013
#
# main structure is adapted from Bandono's
# matrixQPI which is wiringPi based.
# https://github.com/bandono/matrixQPi?source=cc
# #####################################################
import RPi.GPIO as GPIO
import time

class Keypad():
    # CONSTANTS
    KEYPAD = [
        [1, 2, 3],    # 6
        [4, 5, 6],    # 1
        [7, 8, 9],    # 2
        ["*", 0, "#"] # 4
    ]   # 5  7  3

    ROW = [26, 19, 13, 6]  # Row GPIO pins, matrix 5, 7, 3
    COLUMN = [21, 20, 16]  # Column GPIO pins, patrix 6, 1, 2, 4

    def __init__(self):
        GPIO.setmode(GPIO.BCM)
        GPIO.setwarnings(False)  # Disable warnings for GPIO setup

        # Set up rows as inputs with pull-up resistors
        for row in self.ROW:
            GPIO.setup(row, GPIO.IN, pull_up_down=GPIO.PUD_UP)
        
        # Set up columns as outputs
        for col in self.COLUMN:
            GPIO.setup(col, GPIO.OUT)
            GPIO.output(col, GPIO.HIGH)  # Initialize columns as HIGH

    def getKey(self):
        # Scan through rows
        for i in range(len(self.ROW)):
            # Set the current row to output LOW, others stay HIGH
            GPIO.setup(self.ROW[i], GPIO.OUT)
            GPIO.output(self.ROW[i], GPIO.LOW)

            # Check the columns
            for j in range(len(self.COLUMN)):
                if GPIO.input(self.COLUMN[j]) == GPIO.LOW:
                    # Key is pressed
                    print(f"Key detected: {self.KEYPAD[i][j]} (Row: {i}, Column: {j})")  # Debug output
                    GPIO.setup(self.ROW[i], GPIO.IN, pull_up_down=GPIO.PUD_UP)
                    return self.KEYPAD[i][j]

            # Reset the row to input after scanning
            GPIO.setup(self.ROW[i], GPIO.IN, pull_up_down=GPIO.PUD_UP)

        return None  # No key is pressed

    def exit(self):
        # Reinitialize all rows and columns as input at exit
        for i in range(len(self.ROW)):
            GPIO.setup(self.ROW[i], GPIO.IN, pull_up_down=GPIO.PUD_UP)
        for j in range(len(self.COLUMN)):
            GPIO.setup(self.COLUMN[j], GPIO.IN, pull_up_down=GPIO.PUD_UP)

if __name__ == '__main__':
    # Initialize the keypad class
    kp = Keypad()

    # Loop while waiting for a keypress
    digit = None
    while digit is None:
        digit = kp.getKey()
        time.sleep(0.2)  # Debounce delay

    # Print the result
    print(f"Key pressed: {digit}")
    kp.exit()

