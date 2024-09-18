use gtk::prelude::*;

fn main() {
    let app = gtk::Application::new(
        Some("com.github.keens.gtk-examples.basic"),
        Default::default(),
    );

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("My First gtk-rs Program"));
    window.set_default_size(350, 70);

    let sp = 6;
    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .halign(gtk::Align::Start)
        .spacing(sp)
        .margin_bottom(sp)
        .margin_top(sp)
        .margin_start(sp)
        .margin_end(sp)
        .build();

    window.set_child(Some(&vbox));

    let button = build_button(String::from("click me!"));
    vbox.append(&button);

    window.show();
}

fn build_button(label: String) -> gtk::Button {
    let button = gtk::Button::with_label(&label);
    button.connect_clicked(|_| {
        println!("clicked!");
    });
    button
}
