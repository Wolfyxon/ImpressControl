# ImpressProxy
A proxy for LibreOffice Impress Remote that allows the use of WebSocket.

## How to use
### Setting up
1. In LibreOffice Impress go to **Slide Show** > **Slide show settings**
2. Enable remote control
3. Run the program
4. Enter the address (not entering anything and pressing enter will use default settings)
5. Follow the pairing instructions as you'll see
6. Use your WebSocket client to connect and communicate with the presentation

## Why
It allows you to control presentations with browsers, which mostly only support WebSocket.  

## How it works
This program runs a WebSocket server and a TCP client.

Packets received from WebSocket are forwarded to TCP and vice versa.

[Impress Remote Protocol documentation](https://cgit.freedesktop.org/libreoffice/core/tree/sd/README_REMOTE).

