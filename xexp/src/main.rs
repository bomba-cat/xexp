use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Grid};
use std::process::Command;
use std::{str, env};

fn main() {
    let app = Application::builder()
        .application_id("org.xexp.explorer")
        .build();

    reconnect(&app);
}

fn button_function(app: &Application, win: &ApplicationWindow, grid: &Grid, directory: String) {
    match env::set_current_dir(directory) {
        Ok(_) => {
            println!("success!");
            reset_window(win, grid);
            reconnect(app);
        }
        Err(e) => eprintln!("no success: {}",e),
    }
    main();
}

fn reset_window(win: &ApplicationWindow, grid: &Grid) {
    let children = grid.children();
    for child in children {
        grid.remove(&child);
    }
    win.show_all();
}

fn reconnect(app: &Application) {
    app.connect_activate(|app| {
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(720)
            .default_height(480)
            .title("xexp explorer")
            .build();
        
        let mut index = 1;
        let mut indexdotted = 1;

        // Get items in current directory
        let mut get_pwdc = if cfg!(target_os = "windows") {
            Command::new("echo")
        } else {
            Command::new("ls")
        };

        let pwdc_output = get_pwdc.output().expect("failed to to get contents of current directory!");
        let pwdc = String::from_utf8(pwdc_output.stdout).unwrap();

        let pwdc_array: Vec<String> = pwdc.split_whitespace().map(|s| s.to_string()).collect();

        // Get current directory
        let mut get_pwd = if cfg!(target_os = "windows") {
            Command::new("cmd")
        } else {
            Command::new("pwd")
        };

        let pwd_output = get_pwd.output().expect("failed to get pwd!");
        let pwd = str::from_utf8(&pwd_output.stdout).unwrap();

        let grid = Grid::new();
        grid.set_row_spacing(10);
        grid.set_column_spacing(10);
        
        grid.attach(&Label::new(Some(pwd)), 3, 0, 1, 1);

        for item in pwdc_array {
            let button = Button::with_label(item.as_str());
            let item_clone = item.clone();
            let app_clone = app.clone();
            let win_clone = win.clone();
            let grid_clone = grid.clone();

            button.connect_clicked(move |_| {
                button_function(&app_clone, &win_clone, &grid_clone, item_clone.to_string());
            });

            if item.contains(".") {
                indexdotted += 1;
                grid.attach(&Label::new(Some(" | ")), 1, indexdotted, 1, 1);
                grid.attach(&Button::with_label(item.as_str()), 2, indexdotted, 1, 1);
            } else {
                index += 1;
                grid.attach(&Button::with_label(item.as_str()), 0, index, 1, 1);
            }
        }
        
        win.add(&grid);

        win.show_all();
    });

    app.run();
}
