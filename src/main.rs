/*
 * Copyright (c) 2023 Aeonix https://github.com/Aeonix-OHG
 * All Rights Reserved
 * Project: src
 * File: main.rs
 * 
 * Author: Jan Simon Schmitt
 * Created: 31 12 2023
 * Modified: 31 12 2023
 * Modified By: Jan Simon Schmitt
 */

use gfxsrc;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let option = &args[1];
    let package = &args[2]; 
    let mut app = gfxsrc::Screen::new(30, 30, ' '.to_string());
    app.set_title("Package_Crane", "#FFFFFF");
    app.addoutline("#FFFF00");
    app.addstring(3, 4, "Package-Crane", "#FFFFFF");
    app.addstring(6, 5, "by Bonbon", "#FFFFFF");
    app.addstring(10, 24, "app gets installed", "#FFFFFF");
    app.addstring(12, 25, "Please Wait", "#FFFFFF");
    let _ = app.addpic(0, 22, "crane.npf");
    app.print();
}
