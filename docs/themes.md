# Colors and color palettes

The theme struct and all the themes are in the [theme.rs file](https://github.com/S0d4Dev/kagami/blob/main/src/theme.rs).

Each color palette is composed of seven elements :
    
    - The background color (bg) : self explanatory
    - The overlay color (overlay) : used for the shadows of widgets
    - The subtext color (subtext) : used for most of the info widgets
    - The text color (text) : used for the character's name and some info widgets
    - The main accent color (accent) : used for the widgets
    - The secondary accent color (secondary_accent) : darker color, used for the characters and some info widgets
    - The tertiary accent color (tertiary_accent) : lighter color, used for the difficulty slider and some info widgets

The colors are in RGB 565, meaning you will to use an RGB color picker such as [this one](https://rgbcolorpicker.com/565). 