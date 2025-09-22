# ImpressProxy
A proxy for LibreOffice Impress Remote that allows the use of WebSocket.

## How to use
### Setting up
1. In LibreOffice Impress go to **Slide Show** > **Slide show settings**
2. Enable remote control
3. Run the program
4. Enter the address (not entering anything and pressing enter will use default settings)
5. Follow the pairing instructions as you'll see
6. Connect to `ws://localhost:1600` with your code.

Port settings may be available in future.

[Impress Remote Protocol documentation](https://cgit.freedesktop.org/libreoffice/core/tree/sd/README_REMOTE).

## Why
It allows you to control presentations with browsers, which mostly only support WebSocket.  

## How it works
This program runs a WebSocket server and a TCP client.

It forwards packets between WebSocket (`1600`) and Impress Remote TCP (`1599`).
