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


Stole this code from chatbot
"""
import RPi.GPIO as GPIO
import time

# Set up GPIO mode
GPIO.setmode(GPIO.BCM)

# Define GPIO pins for rows and columns
rows = [26, 13, 6, 5]    # GPIO pins for rows
cols = [22, 27, 17]      # GPIO pins for columns

# Keypad mapping (3x4)
keypad = [
    ['1', '2', '3'],
    ['4', '5', '6'],
    ['7', '8', '9'],
    ['*', '0', '#']
]

# Set up the GPIO pins for rows as inputs with pull-ups
for row in rows:
    GPIO.setup(row, GPIO.IN, pull_up_down=GPIO.PUD_UP)

# Set up the GPIO pins for columns as outputs
for col in cols:
    GPIO.setup(col, GPIO.OUT)
    GPIO.output(col, GPIO.HIGH)  # Set columns high initially

# Function to detect key press
def get_key():
    for col_idx, col in enumerate(cols):
        GPIO.output(col, GPIO.LOW)  # Set current column to LOW
        for row_idx, row in enumerate(rows):
            if GPIO.input(row) == GPIO.LOW:  # Check if button is pressed
                GPIO.output(col, GPIO.HIGH)  # Reset column state
                return keypad[row_idx][col_idx]  # Return the key that was pressed
        GPIO.output(col, GPIO.HIGH)  # Reset column state
    return None

try:
    print("Waiting for keypress...")
    while True:
        key = get_key()  # Check for key press
        if key:
            print(f"Key pressed: {key}")
            time.sleep(0.5)  # Debounce delay
        time.sleep(0.1)  # Reduce CPU usage when no key is pressed
finally:
    GPIO.cleanup()  # Clean up GPIO on exit

