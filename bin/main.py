import RPi.GPIO as GPIO
import time
import subprocess

# Set up the GPIO mode to BCM
GPIO.setmode(GPIO.BCM)

# Define the GPIO pin for hook/offhook detection
switch_pin = 16  # GPIO pin for hook/offhook detection (was previously GPIO 12)

# Set the GPIO pin as input for the switch
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
        print("Stopping keypad.py...")
        process.terminate()  # Terminate the running program
        process = None

def is_offhook_state():
    return GPIO.input(switch_pin) == GPIO.LOW  # LOW means offhook, HIGH means onhook

try:
    while True:
        # Check switch state
        if is_offhook_state():  # Switch is "offhook" (closed)
            if not is_offhook:
                print("Offhook detected.")
                run_keypad_program()  # Start the keypad program
                is_offhook = True  # Set the state as offhook
        else:  # Switch is "hook" (open)
            if is_offhook:
                print("Onhook detected.")
                stop_program()  # Stop the keypad program
                is_offhook = False  # Reset the state

        time.sleep(0.1)  # Check every 100ms

except KeyboardInterrupt:
    print("Program exited")

finally:
    # Clean up GPIO settings before exiting
    GPIO.cleanup()  # Clean up GPIO settings

