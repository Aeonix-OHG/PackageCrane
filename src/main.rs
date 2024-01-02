
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    let option = &args[1];
    // let package = &args[2]; 
    if option == "install" {
        let package = &args[2];
    }
    else if option == "search" {
        let package = &args[2];
    }
    else if option == "update" {

    }
    else if option == "upgrade" {

    }
    else if option == "addlist" {
        let list = &args[2];
    }
    else if option == "removelist" {
        let list = &args[2];
    }
    else if option == "remove" {
        let package = &args[2];
    }
    else if option == "crpackage" {
        let package = &args[2];
    }
    else {

    }
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
