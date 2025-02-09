use std::env;
use client_ui::loop_forever_media_gui;
use simulation_controller::loop_forever_sc;
use client_tui::loop_forever_chat_tui;

fn main() {
    println!("{:?}", env::args());
    let bin = env::args().nth(1);
    let extra = env::args().nth(2);

    match bin {
        Some(bin) if bin == "media-gui" => {
            loop_forever_media_gui(extra.expect("NEED addr"));
        },
        Some(bin) if bin == "chat-tui" => {
            let _ = loop_forever_chat_tui(extra.expect("NEED port"));
        },
        _ => loop_forever_sc(),
    }
}
