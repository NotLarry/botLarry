# botLarry
                     botlarry comes with ABSOLUTELY NO WARRANTY.

                                  ________________                       
                                 /   __           \
                                 |  )__( O---  || |                      
                                 |   __________   |
                                 |  |          |  |
                                 |  |   .--.   |  |                      
                                 |     /    \     |                      
                                 |     \    /     |                      
                                 |     I|  |I     |                      
                               oO|     I|  |I     |                      
                              oO`|     I|  |I     |                      
                             oO  |     I|  |I     |                      
                             Oo  |     /    \     |                      
                             oO  |     \    /     |                      
                             Oo  |      Oo-'      |                      
                             oO  |     oO   .---. |                      
                              Oo |    oO    || || |                      
                                OoOOoO      |'-'| |                      
                                 |          '---' |                      
                                 \________________/


TL;DR - botLarry is currently a project that will populate a payphone with audio files from myself, or from my friends.  
The idea being that in this day and age I can leave my family and friends an artifact they can revisit memories of me with.  


 

It was a long long time ago in a galaxy far far awa, er, well, actually it was maybe 2002 and an event called "Hacker Hookup" was scheduled for Phreaknic 6.

Hacker Hookup would involve a specific gender in the auditorium at the Hilton in Nashville talking over irc to a group of prospective dates.  As part of this NotLarry build a small server in a case made of LEGO bricks.  On this server he had constructed an IRC bot that would answer questions asked by the contestants before the hackers could.                                        
                                                                                  
As I remember this was a good time.  As my spawn hdz recalls it was a good time and at a recent 2600 meeting where we were discussing robotics and playing with some parts he had scored we talked about bringing botLarry back to life.
                                                                             
There have been a number of projects under this heading.  The current is a payphone that plays recordings from NotLarry


==update==

1) payphone

  a) 3x4 keypad (7 pin)

  b) off hook switch (2 pins, activates pin 1 on 20 pinout)

  c) volume adjust (2 pins, should set 3 levels of output volume and have triggering this cycles).

  d) handset

    A) 2 pin to mouthpiece, connect to mic input on NUC

    B) 2 pin to earpiece, connect to audio out on NUC

  e) 4 pins for coin reception

  f) 2 pins that rquire 12 volts to trigger the coin collection selinoid

2) power supply that gives 5 and 12 volts

3) relay for the 12 volt selenoid

4) Raspberry pi 4




====phonehope====

botLarry will now become a payphone

The idea is I will have recordings of myself (or others) and when you dial a number it plays an mp3 
Years later my kids can fire it up and hear their fathers voice, creepy right?:)



"Some of this code was generated with the help of OpenAI's ChatGPT."  When I asked the robot how to attribute it, it told me not to worry about it.  
I guess code developed from stollen code is fair game?  I don't think so.  I will do my best to attibute any specific code I get with the help of ChatGPT.


Testing doing this in rust instead of python

On a side note, I'm putting this here for posterity


"NotLarry's origin story.

I had moved into a house with my two best friends.
We checked pets on the lease because I had a ferret (polyester).
When we moved in there was a cross eyed siamese kitten and we joked that they left that because we checked pets on the lease.
We could not agree on a name.
Then  on August 8, 1989 watching Letterman the top 10 list was

Top 10 Demands of the Striking Telephone Workers
 2. The right to call everyone "Larry" -- as in: "Thank you for using
    AT&T, Larry."

I said, we can call him Larry, and my roommate said 'his name is not Larry'
it stuck.
it is now my true identity, and honors a long gone cat."


This should be an up to date list of the GPIO pins in use but 
I am pretty sure it is out of date.




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
12Vrelay ctrl GPIO6       [31] [32]      GPIO12
              GPIO13      [33] [34]      GND ⏚
              GPIO19      [35] [36]      GPIO16    kbd Row 1
HOOK +        GPIO26      [37] [38]      GPIO20
HOOK -        GND ⏚       [39] [40]      GPIO21
