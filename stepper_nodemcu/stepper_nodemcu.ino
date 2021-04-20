/**************************************************************************
 * 
 * ESP8266 NodeMCU stepper motor control with rotary encoder.
 * This is a free software with NO WARRANTY.
 * https://simple-circuit.com/
 *
 *************************************************************************/
 
// include Arduino stepper motor library
#include <Stepper.h>
 
// number of steps per one revolution is 2048 ( = 4096 half steps)
#define STEPS  2048
 
// stepper motor control pins
#define IN1   D1   // IN1 is connected to NodeMCU pin D1 (GPIO5)
#define IN2   D2   // IN2 is connected to NodeMCU pin D2 (GPIO4)
#define IN3   D3   // IN3 is connected to NodeMCU pin D3 (GPIO0)
#define IN4   D4   // IN4 is connected to NodeMCU pin D4 (GPIO2)
 
// initialize stepper library
//Stepper stepper(STEPS, IN4, IN2, IN3, IN1);
//Stepper stepper(STEPS, IN4, IN2, IN3, IN1);
//Stepper stepper(STEPS, IN4, IN2, IN3, IN1);
Stepper stepper(STEPS, IN4, IN3, IN2, IN1);
 
// rotary encoder pins
#define DT    D5   // DT pin, connected to NodeMCU pin D5 (GPIO14)
#define CLK   D6   // CLK pin, connected to NodeMCU pin D5 (GPIO12)
 
int8_t  quad = 0;
uint8_t previous_data;
 
void ICACHE_RAM_ATTR enc_read();
void setup(void)
{
  pinMode(DT,  INPUT);
  pinMode(CLK, INPUT);
  pinMode(IN1, OUTPUT);
  pinMode(IN2, OUTPUT);
  pinMode(IN3, OUTPUT);
  pinMode(IN4, OUTPUT);
  stepper.setSpeed(10);  // set stepper motor speed to 10 rpm
 
  previous_data = digitalRead(DT) << 1 | digitalRead(CLK);
 
  // enable interrupts for rotary encoder pins
  attachInterrupt(DT,  enc_read, CHANGE);
  attachInterrupt(CLK, enc_read, CHANGE);
}
 
void ICACHE_RAM_ATTR enc_read()
{
  uint8_t current_data = digitalRead(DT) << 1 | digitalRead(CLK);
 
  if( current_data == previous_data )
    return;
 
  if( bitRead(current_data, 0) == bitRead(previous_data, 1) )
    quad -= 1;
  else
    quad += 1;
  previous_data = current_data; 
 
}
 
int8_t encoder_update(void)
{
  int8_t val = 0;
  while(quad >= 4){
    val += 1;
    quad -= 4;
  }
  while(quad <= -4){
    val -= 1;
    quad += 4;
  }
  return val;
}
 
// main loop
void loop()
{
  int8_t stp = encoder_update();
 
  while(stp != 0)
  {
    int8_t dir = (stp > 0) ? -1 : 1;
    stepper.step( 200 * dir );
    stp += dir;
    stp += encoder_update();
  }
 
  digitalWrite(IN1, LOW);
  digitalWrite(IN2, LOW);
  digitalWrite(IN3, LOW);
  digitalWrite(IN4, LOW);
 
  delay(100);
}
 
// end of code.
