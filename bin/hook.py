"""
The current thinking behind hook.py is it watches for changes to the swich controlled by the handset
If the phone is hung up the circuit is broken and this does nothing
If the reciever is lifted the circuit closes and we can move forward with other processes.

It is possible that we may watch during onhook for input from keypad but not yet.  Right now I think
it would be best to have all processes off unless this goes offhook.

Also look into python subprocesses
https://docs.python.org/3/library/subprocess.html

"""

#Importing the correct libraries
import RPi.GPIO as GPIO
from time import sleep
GPIO.setmode(GPIO.BCM)

pushpin = 21 
GPIO.setup(pushpin, GPIO.IN, pull_up_down=GPIO.PUD_UP) 

# this test simply watches gpio pushpin and prints the current state.

while True:
           hook = GPIO.input(pushpin)       # Reading the inverse value of input pin 17
           sleep(0.2)
           if hook == 0:
             print('offhook', hook)
           else:
             print('onhook', hook)
