This should be an up to date list of the GPIO pins in use but 
I am pretty sure it is out of date.


coin_collection.py
  GPIO 4
keypad.py
  GPIO 26
  GPIO 13
  GPIO 6
  GPIO 5
  GPIO 22
  GPIO 27
  GPIO 17
hook.py
  GPIO 12  


12Vrelay +    3.3V        [01] [02]      5V
              GPIO2 [SDA] [03] [04]      5V
              GPIO3 [SCL] [05] [06]      GND ⏚
              GPIO4       [07] [08]      GPIO14 [TXD]
              GND ⏚       [09] [10]      GPIO15 [RXD]
kpd Col 3     GPIO17      [11] [12]      GPIO18
kbd Col 2     GPIO27      [13] [14]      GND ⏚
kbd Col 1     GPIO22      [15] [16]      GPIO23    kbd Row 4
              3.3V        [17] [18]      GPIO24    kbd Row 3
              GPIO10      [19] [20]      GND ⏚
              GPIO9       [21] [22]      GPIO 25   kbd Row 2
              GPIO11      [23] [24]      GPIO8
              GND ⏚       [25] [26]      GPIO7
              GPIO0       [27] [28]      GPIO1
              GPIO5       [29] [30]      GND ⏚
 12Vrelay pin GPIO6       [31] [32]      GPIO12
              GPIO13      [33] [34]      GND ⏚
              GPIO19      [35] [36]      GPIO16    kbd Row 1
HOOK +        GPIO26      [37] [38]      GPIO20
HOOK -        GND ⏚       [39] [40]      GPIO21
