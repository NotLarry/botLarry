▌   ▗ ▖         
▛▌▛▌▜▘▌ ▀▌▛▘▛▘▌▌
▙▌▙▌▐▖▙▖█▌▌ ▌ ▙▌
              ▄▌

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


A quick note: The content is going to be the hardest part.  If you know NotLarry and either want to create an anecdote about him, or remind him of something he should add, please reach out to notlarry@hotmmail.com


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


This should be an up to date list of the GPIO  pins in use but 
I am pretty sure it is out of date.



Raspberry pi 5 pinout

<pre>

12Vrelay pwr-----3.3V                  [01] [02]      5V Power
                 GPIO 2 (SDA)          [03] [04]      5V Power
                 GPIO 3 (SCL)          [05] [06]      GND ⏚
                 GPIO 4 (GPCLK)        [07] [08]      GPIO 14 (TXD)
                 GND ⏚                 [09] [10]      GPIO 15 (RXD)
kpd Col 3--------GPIO 17               [11] [12]      GPIO 18 (PCM_CLK)
kbd Col 2--------GPIO 27               [13] [14]      GND ⏚
kbd Col 1--------GPIO 22               [15] [16]      GPIO 23-------kbd Row 4
                 3.3V                  [17] [18]      GPIO 24.......kbd Row 3
                 GPIO 10 (MOSI)        [19] [20]      GND ⏚
                 GPIO 9  (MISO)        [21] [22]      GPIO  25------kbd Row 2
                 GPIO 11 (SCLK)        [23] [24]      GPIO 8  (CE0)
Volume - --------GND ⏚                 [25] [26]      GPIO 7  (CE1)
                 GPIO 0  (ID_SD        [27] [28]      GPIO 1  (ID_SC)
Volume control---GPIO 5                [29] [30]      GND ⏚
12Vrelay ctrl----GPIO 6                [31] [32]      GPIO 12 (PWM0)
                 GPIO 13 (PWM1)        [33] [34]      GND ⏚
                 GPIO 19 (PCM_FS)      [35] [36]      GPIO 16-------kbd Row 1
HOOK +   --------GPIO 26               [37] [38]      GPIO 20 (PCM_din)
HOOK -   --------GND ⏚                 [39] [40]      GPIO 21 (PCM_dout)
</pre>


Telephone Pinout

<pre>

                  Red Hook             [01] [02]      Red Receiver      
                  Black Hook           [03] [04]      Yellow Mic 
                  Green Receiver       [05] [06]      Keyboard 6 (column) 
                  J6                   [07] [08]      Keyboard 1 (column)      
                  Keyboard 5 (row)     [09] [10]      Keyboard 2 (column)      
                  Keyboard 7 (row)     [11] [12]      Keyboard 4 (column)      
                  Keyboard 3 (row)     [13] [14]      
                                       [15] [16]      
                                       [17] [18]      
                  Piezio +             [19] [20]      Piezio - 
</pre>
"""

apt/history.log.1.gz:Commandline: apt install smartmontools
apt/history.log.1.gz:Commandline: apt install screen
apt/history.log.1.gz:Commandline: apt install mc -y
apt/history.log.1.gz:Commandline: apt install mpg123
apt/history.log.1.gz:Commandline: apt install tmux
apt/history.log.1.gz:Commandline: apt install sox libsox-fmt-alsa
apt/history.log.2.gz:Commandline: apt install libsdl2-2.0-0 libsdl2-dev libsndfile1
apt/history.log.2.gz:Commandline: apt install telnet
apt/history.log.2.gz:Commandline: apt install nmap
apt/history.log.2.gz:Commandline: apt install ffmpeg
apt/history.log.2.gz:Commandline: apt install hostapd dnsmasq
apt/history.log.2.gz:Commandline: apt-get install dphys-swapfile
apt/history.log.3.gz:Commandline: apt install git
apt/history.log.3.gz:Commandline: apt install vim
apt/history.log.3.gz:Commandline: apt install sqlite
apt/history.log.3.gz:Commandline: apt install btop

