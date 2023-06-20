extern crate gtk;

use gtk::{prelude::*, glib};

#[derive(Debug)]
enum UiEvent {
    SwitchView(View),
    ButtonClicked(String),
    Processed(String),
    Submit,
    Clear,
    Exit,
}

#[derive(Debug)]
enum View {
    Step1,
    Step2,
    Step3,
}

struct ApplicationState {
    // Add any necessary fields for the app state
    pub state: Vec<String>
}

impl ApplicationState {
    fn new() -> Self {
        Self {
            // Initialize the state as needed
            state: Vec::new()
        }
    }

    fn switch_view(&self, view: View) {
        // Logic for switching views
        println!("Switching to view: {:?}", view);
    }

    fn processing(&self, data: &String) {
        // Logic for showing processing state
        println!("Processing data: {}", data);
    }

    fn processed(&mut self, data: String) {
        // Logic for showing processing state
        println!("Processed data: {}!!!", data);
        self.state.push(data);
    }

    fn submit(&mut self) {
        println!("Submitted data: {:?}", self.state);
        self.state.clear();
    }

    fn clear(&mut self) {
        self.state.clear();
    }

}

fn background_event_loop(tx: glib::Sender<UiEvent>, rx: std::sync::mpsc::Receiver<String>) {
    std::thread::spawn(move || {
        for data in rx {
            // Simulating some background processing
            std::thread::sleep(std::time::Duration::from_secs(5));
            tx.send(UiEvent::Processed(data)).expect("Failed to send Processed event.");
        }
    });
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // Creating channels
    let (tx_background, rx_background) = std::sync::mpsc::channel();
    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);

    // Start the background thread
    background_event_loop(tx.clone(), rx_background);

    // Begin create Window controls
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("GTK Channel Example");
    window.set_default_size(400, 80);
    window.set_border_width(10);

    let container = gtk::Box::new(gtk::Orientation::Vertical, 5);
    window.add(&container);

    let text_entry = gtk::Entry::new();
    container.add(&text_entry);
    
    let button = gtk::Button::with_label("Add Text");
    container.add(&button);
    
    let clear = gtk::Button::with_label("Clear");
    container.add(&clear);

    let label_container = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    container.add(&label_container);

    let msg_label = gtk::Label::new(Some("To be Submitted:"));
    label_container.add(&msg_label);

    let list_label = gtk::Label::new(None);
    label_container.add(&list_label);
    
    let inner_container = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    inner_container.set_homogeneous(true);
    container.add(&inner_container);
    
    let close = gtk::Button::with_label("Close");
    inner_container.add(&close);

    let submit = gtk::Button::with_label("Submit");
    inner_container.add(&submit);
    // End create Window controls

    // Begin setup control events
    let tx_clone = tx.clone();
    let text_entry_clone = text_entry.clone();
    button.connect_clicked(move |_| {
        tx_clone.send(UiEvent::ButtonClicked(text_entry_clone.text().to_string())).unwrap();
    });

    let tx_clone = tx.clone();
    submit.connect_clicked(move |_| {
        tx_clone.send(UiEvent::Submit).unwrap();
        tx_clone.send(UiEvent::SwitchView(View::Step2)).unwrap();
    });

    let tx_clone = tx.clone();
    let text_entry_clone = text_entry.clone();
    clear.connect_clicked(move |_| {
        text_entry_clone.set_text("");
        tx_clone.send(UiEvent::Clear).unwrap();
    });

    let tx_clone = tx.clone();
    close.connect_clicked(move |_| {
        tx_clone.send(UiEvent::Exit).unwrap();
    });
    // End setup control events

    // Setup application state
    let mut app_state = ApplicationState::new();

    // Setup application main loop
    rx.attach(None, move |event| {
        match event {
            UiEvent::SwitchView(view) => app_state.switch_view(view),
            UiEvent::ButtonClicked(data) => {
                app_state.processing(&data);
                tx_background.send(data).expect("Failed to send data to background thread.");
            }
            UiEvent::Processed(data) => {
                app_state.processed(data);
                list_label.set_text(app_state.state.join(",").as_str());
            }
            UiEvent::Submit => {
                app_state.submit();
                list_label.set_text("");
            }
            UiEvent::Clear => {
                app_state.clear();
                list_label.set_text("");
            }
            UiEvent::Exit => {
                gtk::main_quit();
                return Continue(false);
            }
        }
        Continue(true)
    });

    // Window close exits the app
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Window show all controls visible
    window.show_all();

    // Starts the gtk app
    gtk::main();
}

