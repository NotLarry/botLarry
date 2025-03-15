import time
import signal
import sys
from pyftdi.gpio import GpioController

# Function to handle Ctrl+C and clean up GPIOs properly
def signal_handler(sig, frame):
    print("\nExiting...")
    gpio.close()
    sys.exit(0)

# Register the signal handler for Ctrl+C
signal.signal(signal.SIGINT, signal_handler)

# Create an FTDI GPIO controller object
gpio = GpioController()

# Open the FT232H device (adjust the FT232H URL if needed)
gpio.open_from_url('ftdi://ftdi:232h/1')

# Define the relevant pins
pins = {
    'C0': 0,  # C0 (pin 0)
    'C1': 1,  # C1 (pin 1)
    'C3': 3   # C3 (pin 3)
}

# Set C0, C1, and C3 as output
gpio.set_direction(pins['C0'], 0)  # C0 as output
gpio.set_direction(pins['C1'], 0)  # C1 as output
gpio.set_direction(pins['C3'], 0)  # C3 as output

# Set C0, C1, and C3 to low initially
gpio.write(0)  # Set all pins to low (0 means low)

# To remember the previous state of C0, C1 and C3
prev_c0_state = None
prev_c1_state = None
prev_c3_state = None

# Continuously check if C0 and C1 are shorted and update C3
try:
    while True:
        # Read the states of C0 and C1
        state_c0 = gpio.read(pins['C0'])
        state_c1 = gpio.read(pins['C1'])

        # If C0 and C1 are both low, they are shorted (continuity)
        if state_c0 == 0 and state_c1 == 0:
            gpio.write(0)  # Set C3 low (0 means low)
            current_c3_state = "low"
        else:
            gpio.write(1 << pins['C3'])  # Set C3 high (1 << 3 means high)
            current_c3_state = "high"

        # Print if C3 state has changed
        if current_c3_state != prev_c3_state:
            print(f"C3 is now {current_c3_state}")
            prev_c3_state = current_c3_state

        # Monitor and print any changes in C0 or C1
        if state_c0 != prev_c0_state:
            print(f"C0 state changed to {'HIGH' if state_c0 else 'LOW'}")
            prev_c0_state = state_c0

        if state_c1 != prev_c1_state:
            print(f"C1 state changed to {'HIGH' if state_c1 else 'LOW'}")
            prev_c1_state = state_c1

        # Sleep for a short period to avoid excessive polling
        time.sleep(0.1)

except KeyboardInterrupt:
    # Gracefully handle the keyboard interrupt (Ctrl+C)
    signal_handler(None, None)

