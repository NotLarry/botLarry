import RPi.GPIO as GPIO
import time

# Set the GPIO mode
GPIO.setmode(GPIO.BCM)

# Set up GPIO4 as an output pin
GPIO.setup(4, GPIO.OUT)

# Function to activate the relay (turn it on)
def activate_relay():
    GPIO.output(4, GPIO.HIGH)  # Send HIGH signal (turns relay on)

# Function to deactivate the relay (turn it off)
def deactivate_relay():
    GPIO.output(4, GPIO.LOW)   # Send LOW signal (turns relay off)

# Test the relay by activating and deactivating it
try:
    while True:
        activate_relay()   # Turn relay on
        print("Relay Activated")
        time.sleep(2)      # Wait for 2 seconds

        deactivate_relay() # Turn relay off
        print("Relay Deactivated")
        time.sleep(2)      # Wait for 2 seconds

except KeyboardInterrupt:
    # Clean up GPIO settings on exit
    GPIO.cleanup()
    print("Exiting and cleaning up GPIO")
