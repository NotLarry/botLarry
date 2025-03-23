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

class keypad():
    # CONSTANTS   
    KEYPAD = [
    [1,2,3],
    [4,5,6],
    [7,8,9],
    ["*",0,"#"]
    ]
    
    ROW         = [26,19,13,6]
    COLUMN      = [21,20,16]
    
    def __init__(self):
        GPIO.setmode(GPIO.BCM)
    
    def getKey(self):
        
        # Set all columns as output low
        for j in range(len(self.COLUMN)):
            GPIO.setup(self.COLUMN[j], GPIO.OUT)
            GPIO.output(self.COLUMN[j], GPIO.LOW)
        
        # Set all rows as input
        for i in range(len(self.ROW)):
            GPIO.setup(self.ROW[i], GPIO.IN, pull_up_down=GPIO.PUD_UP)
        
        # Scan rows for pushed key/button
        # A valid key press should set "rowVal"  between 0 and 3.
        rowVal = -1
        for i in range(len(self.ROW)):
            tmpRead = GPIO.input(self.ROW[i])
            if tmpRead == 0:
                rowVal = i
                
        # if rowVal is not between 0 and 3 (inclusive), then no button was pressed and we can exit
        if rowVal < 0 or rowVal > 3:
          self.exit()
        return

        
        # Convert columns to input
        for j in range(len(self.COLUMN)):
                GPIO.setup(self.COLUMN[j], GPIO.IN, pull_up_down=GPIO.PUD_DOWN)
        
        # Switch the i-th row found from scan to output
        GPIO.setup(self.ROW[rowVal], GPIO.OUT)
        GPIO.output(self.ROW[rowVal], GPIO.HIGH)

        # Scan columns for still-pushed key/button
        # A valid key press should set "colVal"  between 0 and 2.
        colVal = -1
        for j in range(len(self.COLUMN)):
            tmpRead = GPIO.input(self.COLUMN[j])
            if tmpRead == 1:
                colVal=j
                
        # if colVal is not between 0 and 2 (inclusive), then no button was pressed and we can exit
        if colVal < 0 or colVal > 2:
          self.exit()
        return

        # Return the value of the key pressed
        self.exit()
        return self.KEYPAD[rowVal][colVal]
        
    def exit(self):
        # Reinitialize all rows and columns as input at exit
        for i in range(len(self.ROW)):
                GPIO.setup(self.ROW[i], GPIO.IN, pull_up_down=GPIO.PUD_UP) 
        for j in range(len(self.COLUMN)):
                GPIO.setup(self.COLUMN[j], GPIO.IN, pull_up_down=GPIO.PUD_UP)
        
if __name__ == '__main__':
    # Initialize the keypad class
    kp = keypad()
    
    # Loop while waiting for a keypress
    digit = None
    while digit == None:
        digit = kp.getKey()
    
    # Print the result
    print(digit)




