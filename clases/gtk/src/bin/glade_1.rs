use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("glade1.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let window: gtk::Window = builder.object("window1").unwrap();
    let button: gtk::Button = builder.object("button1").unwrap();
    let dialog: gtk::MessageDialog = builder.object("messagedialog1").unwrap();

    button.connect_clicked(move |_| {
        dialog.run();
        dialog.hide();
    });

    window.show_all();

    gtk::main();
}
