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

# Function to collect and save 10 digits
def collect_digits():
    digits = []
    while len(digits) < 10:
        key = get_key()  # Check for key press
        if key and key.isdigit():  # Only collect digit keys
            digits.append(key)
            print(f"Key pressed: {key}")
            time.sleep(0.5)  # Debounce delay to prevent multiple presses
        time.sleep(0.1)  # Reduce CPU usage when no key is pressed
    
    # Print and save the collected digits
    print(f"Digits recorded: {''.join(digits)}")
    with open("digits.txt", "a") as file:
        file.write("".join(digits) + "\n")
    
    return digits

try:
    print("Waiting for 10 digits...")
    while True:
        collect_digits()  # Collect 10 digits
        print("Waiting for the next 10 digits...")
        time.sleep(1)  # Wait for a second before starting the next round

finally:
    GPIO.cleanup()  # Clean up GPIO on exit

