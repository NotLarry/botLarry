#include <Servo.h> // including servo library.
/*
 * This is for 3 wire servos
 * Black -ve (ground)
 * Red   +ve (power)
 * White  signal
 * 
 */
Servo servo_1; // Giving name to servo.
Servo servo_2; // Giving name to servo.
int potValue = 0;
int oldValue = 0;
 
void setup (){
  servo_1.attach(5); // Attaching Servo to D3
  servo_2.attach(4); // Attaching Servo to D4
  Serial.begin(115200);
  delay(10);
}

void loop(){

     potValue = analogRead(A0);
     Serial.println("VALUE: "+String(potValue));
     delay (500);
     if (potValue != oldValue) {
      
    

  servo_1.write (145); // Servo will move to 145 degree angle.
    Serial.println("Servo 1 is moved to 145");
  delay (400);
  servo_1.write (20); //Servo will move to 20 degree angle
  delay (400);
    Serial.println("Servo 1 is moved back to 20");
  servo_2.write (20);  // servo will move to 90 degree angle.
    Serial.println("Servo 2 is moved 20");
  delay (400);
  servo_2.write (145);  // servo will move to 145 degree angle.
    Serial.println("Servo 2 is moved 145");
  delay (400);
     }
     else
     {
  servo_1.write (0);
  servo_2.write (0);
}
oldValue = potValue;
}
