#include <Arduino.h>


#define axisX A5
#define axisY A6


void setup() {
  Serial.begin(9600);
  pinMode(axisY,INPUT);
  pinMode(axisX,INPUT);
  pinMode(LED_BUILTIN, OUTPUT);
  digitalWrite(LED_BUILTIN, LOW);

}

int axisYval = 0;
int axisXval = 0;
void loop() {

  String message = readSerial();
  axisXval = analogRead(axisX);
  axisYval = analogRead(axisY);

  if (message == "SEND"){
    String axisMessage = String(axisXval) + " " + String(axisYval);
    sendMessage(axisMessage);
  }
  

  
 
}

String readSerial(){
  String message = "";
  String lastByte = "";
  while (lastByte != "/"){
    if (Serial.available() <= 0){
      continue ;
    }
    lastByte = String((char)Serial.read());
    if (lastByte == "/"){
      return message ;
    }
    message += lastByte;
  }
  return message;
  
}

void sendMessage(String message){
  char messageToSend[message.length() + 1];
  message.toCharArray(messageToSend,message.length()+1);

  Serial.write(messageToSend);
}