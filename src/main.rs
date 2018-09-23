use {
    self::Msg::Quit,
    gtk::prelude::*,
    relm::{connect, connect_stream, Widget},
    relm_attributes::widget,
    relm_derive::Msg,
};

#[derive(Msg)]
pub enum Msg {
    Quit,
}

#[widget]
impl Widget for Win {
    fn model() {}

    fn update(&mut self, msg: Msg) {
        match msg {
            Quit => gtk::main_quit(),
        }
    }

    view! {
        gtk::Window {
            property_width_request: 300,
            resizable: false,
            title: "Relm Hello",

            gtk::Box {
                orientation: gtk::Orientation::Vertical,

                gtk::Label {
                    text: "Hello, world!",
                },
                gtk::Button {
                    clicked => Quit,
                    label: "Close",
                },
            },

            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}
