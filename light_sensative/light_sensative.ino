/* So, comments at the beginning, that is a novel idea.

 lets save the effort to watch for lights.
*/

int adcvalue = 0; // initialize the light sensor value

int relayRed = 12; // Initialize red relay to d4
int relayBlue = 13; // Initialize blue relay to d3
int relayGreen = 14; // Initialize green relay to d2
int randomish = 0; // Fine, I'll initialize my fucking variables
int relayColor = 0;
int relayState = 0;
 
void setup (){
  randomSeed(analogRead(0));
  Serial.begin(115200); // Setup the serial port for 115200 baud, w00t!
  delay(10);
  pinMode(relayRed, OUTPUT); // initialize Red as OUTPUT
  pinMode(relayBlue, OUTPUT); // initialize Blue as OUTPUT
  pinMode(relayGreen, OUTPUT); // initialize Green as OUTPUT

  digitalWrite(relayRed, LOW); // set Red to LOW
  digitalWrite(relayBlue, LOW); // set Blue to LOW
  digitalWrite(relayGreen, LOW); // set Green to LOW
}

void loop(){  // Lets really get started by watching the light


     adcvalue = analogRead(A0);
     Serial.println("VALUE: "+String(adcvalue));
     if (adcvalue < 300) {  // If the sensor sees light lets spring into action
    
    digitalWrite(relayRed, LOW);
    digitalWrite(relayBlue, LOW);
    digitalWrite(relayGreen, LOW);
      
     }
     else // Ohterwise try this
     {
    relayColor = random(12,15);  
   
    relayState = random(0,2);
    delay(200);
    digitalWrite(relayColor, relayState);
    
}
}
/* 
notes and code I'm not using yet;

 
 */
