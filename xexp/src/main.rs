use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Grid};
use std::process::Command;

fn main() {
    let mut pwd = if cfg!(target_os = "windows") {
        Command::new("dir")
    } else {
        Command::new("ls")
    };

    pwd.status().expect("failed to execute command!");

    println!();

    let app = Application::builder()
        .application_id("org.xexp.explorer")
        .build();

    app.connect_activate(|app| {
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(720)
            .default_height(480)
            .title("xexp explorer")
            .build();
 
        let button1 = Button::with_label("1");
        button1.connect_clicked(|_| {
            eprintln!("1");
        });

        let button2 = Button::with_label("2");
        button2.connect_clicked(|_| {
            eprintln!("2");
        });

        let grid = Grid::new();
        grid.set_row_spacing(10);
        grid.set_column_spacing(10);
        
        for index in 0..10 {
            grid.attach(&Label::new(Some("Click me!")), index, 0, 1, 1);
            grid.attach(&Button::with_label("(me)"), index, 1, 1, 1);
        }
        
        win.add(&grid);

        win.show_all();
    });

    app.run();
}
