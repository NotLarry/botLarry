import time
from pyftdi.gpio import GpioController

# Create an FTDI GPIO controller object
gpio = GpioController()

# Configure the FT232H device, assuming it's connected to the first available device
gpio.open_from_url('ftdi://ftdi:232h/1')

# Set up the columns and rows using C0 to C2 for columns, and C3 to C6 for rows
cols = [3, 1, 5]  # C0 to C2 for columns
rows = [2, 7, 6, 4]  # C3 to C6 for rows

# Keypad layout
keys = ((1, 2, 3),
        (4, 5, 6),
        (7, 8, 9),
        ('*', 0, '#'))

# Set all columns as input pins (1 for input)
for col in cols:
    gpio.set_direction(col, 1)

# Set all rows as output pins (0 for output)
for row in rows:
    gpio.set_direction(row, 0)

# Function to scan the keypad
def scan_keypad():
    for row in range(4):
        # Set all rows to high first (resetting previous row state)
        gpio.write(0x0)  # Reset all rows to high (0)

        # Set the current row to low (active low row scanning)
        gpio.write(1 << rows[row])  # Set only the current row to low

        time.sleep(0.01)  # Give it some time to stabilize

        for col in range(3):  # Loop over columns C0 to C2 (3 columns)
            if gpio.read(cols[col]) == 0:  # If the column is low, a key is pressed
                print(f"Key pressed: {keys[row][col]}")  # Print the key value
                while gpio.read(cols[col]) == 0:  # Wait until key is released
                    pass
        # No need to reset rows to high as we do it before every row scan

# Continuously scan for key presses
while True:
    scan_keypad()
    time.sleep(0.1)

