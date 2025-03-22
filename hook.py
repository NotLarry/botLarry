#Importing the correct libraries
import RPi.GPIO as GPIO
from time import sleep
GPIO.setmode(GPIO.BCM)

pushpin = 17
GPIO.setup(pushpin, GPIO.IN, pull_up_down=GPIO.PUD_UP) 

while True:
           GPIO.output(not GPIO.input(pushpin))       # Reading the inverse value of input pin 17
           sleep(0.2)

print('foo')
