# A japanese character table

This project is a simple Japanese character table with a nice interface and features, to run on a Numworks Calculator.

*The project is still in developement, and all the characters and features aren't included. Some already done features, such as the theme, may change in future versions.*

## Documentation
If you want to, feel free to browse the code.

The hiragana, katakana and other characters (such as the comma) are available in the `src/characters` folder.

The core program files lie in the `src/core` folder, and uses **Eadk**, a custom library made by Numworks to interact with the calculator. 

## Get the app
In order to get the app, you can download the .nwa file from the **Releases page** or build it yourself :

### Build it yourself
1. Clone this repository : `git clone https://github.com/S0d4D3v/kagami`
2. Go to the project folder : `cd kagami`
3. Install cargo with [rustup](https://rustup.rs/)
4. Build : `cargo build`

Go to [Numworks's app installer](https://my.numworks.com/apps), connect your calculator and find the Kagami app (either with .nwa if you downloaded it, or with no extension if built yourself).

## License
This project uses the MIT license. 
A copy of the license can be found [here](https://github.com/S0d4D3v/kagami/blob/master/LICENSE).

The Numworks calculator and the Epsilon operating system are Registered Trademarks from Numworks, and the `build.rs` and `.cargo/config.toml` files are from their [rust sample app](https://github.com/numworks/epsilon-sample-app-rust), under the **BSD-3 Clause license**. 
The complete license is available both [in their repository](https://github.com/numworks/epsilon-sample-app-rust/blob/master/LICENSE) and [here](https://github.com/S0d4D3v/kagami/blob/master/NW_LICENSE).