# ImpressControl
A key based controller for LibreOffice Impress designed mostly to change slides of a presentation without having the window focused.

## How to use
### Setting up
1. In LibreOffice Impress go to **Slide Show** > **Slide show settings**
2. Enable remote control
3. Run the program
4. Enter the address (not entering anything and pressing enter will use default settings)
5. Follow the pairing instructions as you'll see
6. Done!

### Controls
- `Q` - previous slide/animation
- `E` - next slide/animation

I may add customization in future updates. 
I didn't put much effort into this as I only needed this program for one quick thing.

## How it works:
It uses a TCP connection to connect to Impress Remote.

See the [protocol documentation](https://cgit.freedesktop.org/libreoffice/core/tree/sd/README_REMOTE) for details.
