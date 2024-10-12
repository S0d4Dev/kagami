# Colors and color palettes

*This documentation is subject to change*

The theme struct and all the themes are in the [theme.rs file](https://github.com/S0d4Dev/kagami/blob/main/src/theme.rs).

### The logic behind the themes

When the program starts, it retrieves both the default color palette using the function `theme::theme()` with the parameter `None` and the number of color palettes using the `theme::themecount()` function.

Whenever the program needs a new color palette (when switching themes), it will use the same function, but this time with the parameter `Some()` and calculate what should be in it.

### Theme structure

Each color palette is composed of seven elements :
    
    - The name (name) : self explanatory
    - The id (id) : used to differenciate and change color palettes
    - The background color (bg) : self explanatory too
    - The overlay color (overlay) : used for the shadows of widgets
    - The subtext color (subtext) : used for most of the info widgets
    - The text color (text) : used for the character's name and some info widgets
    - The main accent color (accent) : used for the widgets
    - The secondary accent color (secondary_accent) : darker color, used for the characters and some info widgets
    - The tertiary accent color (tertiary_accent) : lighter color, used for the difficulty slider and some info widgets

#### RGB 565
The colors are in RGB 565, meaning you will need to use an RGB color picker / converter such as [this one](https://rgbcolorpicker.com/565). 

### How to create your own color palette
To create your own palette, go to the theme.rs file. Add one to the returned value of the function `themecount()`.

Then, in the `match` section of the `theme()` function, go to the line where the comment `// Other themes` is present. Copy the following chunk of code and paste it there :
```rs
Some(5) => Theme {
    name : "Numworks Light\0",
    id : 0,
    bg : 0xffff,
    text : 0x0000,
    subtext: 0x1082,
    overlay : 0xe71c,
    accent : 0xfde9,
    secondary_accent : 0xb46d,
    tertiary_accent : 0xfed3,
},
```

If there is already a 6th theme, increase the number in the `Some()`.

Change all the values that are inside to your liking (take into account that the colors are in the **RGB565** format).

You have now made your own theme!

