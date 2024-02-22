use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Grid};

fn main() {
    let app = Application::builder()
        .application_id("org.xexp.explorer")
        .build();

    app.connect_activate(|app| {
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("xexp explorer")
            .build();
        
        let grid = Grid::new();
        grid.set_row_spacing(10);
        grid.set_column_spacing(10);
        grid.attach(&Button::with_label("Button 1"), 0, 0, 1, 1);
        grid.attach(&Button::with_label("Button 2"), 1, 0, 1, 1);

        win.show_all();
    });

    app.run();
}
