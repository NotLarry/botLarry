#!/home/botlarry/phonehome/bin/python3
"""

  botlarry.py initial entry point for botlarry.  At this time it is assumed this will be the code started by systemctl at startup and will continue to run.

  ### build out a unit file so that this always runs

"""

import RPi.GPIO as GPIO
import time
import subprocess

# Set up the GPIO mode to BCM
GPIO.setmode(GPIO.BCM)

# Define the GPIO pin
switch_pin = 12

# Set the GPIO pin as input
GPIO.setup(switch_pin, GPIO.IN, pull_up_down=GPIO.PUD_UP)  # Using internal pull-up resistor

# Flag to track if we're currently running keypad.py
is_offhook = False
process = None

def run_keypad_program():
    global process
    if process is None:
        print("Running keypad.py...")
        process = subprocess.Popen(["python3", "keypad.py"])  # Adjust with the actual path if needed

def stop_program():
    global process
    if process is not None:
        print("Stopping keypoad.py...")
        process.terminate()  # Terminate the running program
        process = None

try:
    while True:
        # Check switch state
        if GPIO.input(switch_pin) == GPIO.HIGH:  # Switch is "hook" (open)
#            print("hook")
            if is_offhook:
                stop_program()  # If we were offhook, stop the program
                is_offhook = False  # Reset the state
        else:  # Switch is "offhook" (closed)
#            print("offhook")
            if not is_offhook:
                run_keypad_program()  # If we weren't offhook, start the program
                is_offhook = True  # Set the state as offhook

        time.sleep(0.1)  # Check every 100ms

except KeyboardInterrupt:
    print("Program exited")

finally:
    # Clean up GPIO settings before exiting
    GPIO.cleanup()


