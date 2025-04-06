import RPi.GPIO as GPIO
import time

# Set up the GPIO mode to BCM
GPIO.setmode(GPIO.BCM)

# Define the GPIO pin
switch_pin = 12

# Set the GPIO pin as input
GPIO.setup(switch_pin, GPIO.IN, pull_up_down=GPIO.PUD_UP)  # Using internal pull-up resistor

try:
    while True:
                    # Read the switch state
        if GPIO.input(switch_pin) == GPIO.HIGH:
            print("open")
        else:
            print("closed")
        time.sleep(0.1)  # Check the state every 100ms

except KeyboardInterrupt:
    print("Program exited")

finally:
        # Clean up GPIO settings before exiting
    GPIO.cleanup()
