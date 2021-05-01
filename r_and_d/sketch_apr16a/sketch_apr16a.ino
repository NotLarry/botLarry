#include <ESP8266WiFi.h>

const char* ssid = "ssid";
const char* password = "wifi-password";
 
int relayRed = 2; //red  to d4
int relayBlue = 0; // blue to d3
int relayGreen = 5; // green to d2


WiFiServer server(80); // set to ssl port

void setup() {
  Serial.begin(115200);
  delay(10);
 
  pinMode(relayRed, OUTPUT); // initialize Red as OUTPUT
  pinMode(relayBlue, OUTPUT); // initialize Blue as OUTPUT
  pinMode(relayGreen, OUTPUT); // initialize Green as OUTPUT

  digitalWrite(relayRed, LOW); // set Red to LOW
  digitalWrite(relayBlue, LOW); // set Blue to LOW
  digitalWrite(relayGreen, LOW); // set Green to LOW
 
  // Connect to WiFi network
  Serial.println();
  Serial.println();
  Serial.print("Connecting to ");
  Serial.println(ssid);
 
  WiFi.begin(ssid, password);
 
  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");
  }
  Serial.println("");
  Serial.println("WiFi connected");
 
  // Start the server
  server.begin();
  Serial.println("Server started")
 
  // Print the IP address
  Serial.print("Use this URL to connect: ");
  Serial.print("http://");
  Serial.print(WiFi.localIP());
  Serial.println("/");
 
}
 
void loop() {
  // Check if a client has connected
  WiFiClient client = server.available();
  if (!client) {
    return;
  }
 
  // Wait until the client sends some data
  Serial.println("new client");
  while(!client.available()){
    delay(1);
  }
 
  // Read the first line of the request
  String request = client.readStringUntil('\r');
  Serial.println(request);
  client.flush();
 
  // Match the request
 
  if (request.indexOf("/RED=ON") != -1)  {
    digitalWrite(relayRed, HIGH);
  }
  if (request.indexOf("/RED=OFF") != -1)  {
    digitalWrite(relayRed, LOW);
  }
  if (request.indexOf("/BLUE=ON") != -1)  {
    digitalWrite(relayBlue, HIGH);
  }
  if (request.indexOf("/BLUE=OFF") != -1)  {
    digitalWrite(relayBlue, LOW);
  }
   if (request.indexOf("/GREEN=ON") != -1)  {
    digitalWrite(relayGreen, HIGH);
  }
  if (request.indexOf("/GREEN=OFF") != -1)  {
    digitalWrite(relayGreen, LOW);
  }
 
// Set ledPin according to the request
//digitalWrite(ledPin, value);
 
  // Return the response
  client.println("HTTP/1.1 200 OK");
  client.println("Content-Type: text/html");
  client.println(""); //  do not forget this one
  client.println("<!DOCTYPE HTML>");
  client.println("<head>");
  client.println("<meta name='viewport' content='width=device-width, initial-scale=1' />");
  client.println("</head>");
  client.println("<html>");
  
 
  client.println("<br><br>");
  client.println("<a href=\"/RED=ON\"\"><button> Red On  </button></a>");
  client.println("<a href=\"/RED=OFF\"\"><button> Red Off  </button></a><br />");  
  client.println("<a href=\"/BLUE=ON\"\"><button> Blue On </button></a>");
  client.println("<a href=\"/BLUE=OFF\"\"><button> Blue Off </button></a><br />");  
  client.println("<a href=\"/GREEN=ON\"\"><button>Green On </button></a>");
  client.println("<a href=\"/GREEN=OFF\"\"><button>Green Off </button></a><br />");  
  client.println("<br><br>");
 
  client.println("</html>");
 
  delay(1);
  Serial.println("Client disonnected");
  Serial.println("");
 
}
