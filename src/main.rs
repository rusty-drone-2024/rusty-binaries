use client::Client as ChatClient;
use client_tui::loop_forever_chat_tui;
use client_ui::loop_forever_media_gui;
use common_structs::leaf::Leaf;
use matteo_contribution as mc;
use network_initializer::factory::DroneFactory;
use network_initializer::factory::DroneRunnable;
use network_initializer::factory::LeafFactory;
use network_initializer::factory::LeafRunnable;
use network_initializer::{drone_factories, leaf_factories, NetworkInitializer};
use rusty_drones::RustyDrone;
use rusty_drones_servers::{ChatServer, MediaServer, TextServer};
use simulation_controller::loop_forever_sc;
use std::env;
use wg_2024::drone::Drone;

fn main() {
    let bin = env::args().nth(1);
    let extra = env::args().nth(2);

    match bin {
        Some(bin) if bin == "media-gui" => {
            loop_forever_media_gui(extra.expect("NEED addr"));
        }
        Some(bin) if bin == "chat-tui" => {
            let _ = loop_forever_chat_tui(extra.expect("NEED port"));
        }
        _ => start_simulation(),
    }
}

fn start_simulation() {
    let drone_factories = drone_factories!(RustyDrone);
    /*let drone_factories = drone_factories!(
        RustafarianDrone,
        DrOnes,
        FungiDrone,
        DRONEDrone,
        CppEnjoyersDrone,
        LockheedRustin,
        LeDronJames,
        BagelBomber,
        RustDrone,
        RustBustersDrone
    );*/

    let client_factories = leaf_factories!(mc::TextMediaClient, ChatClient, ChatClient);
    let server_factories = leaf_factories!(
        mc::TextServer,
        mc::MediaServer,
        TextServer,
        MediaServer,
        ChatServer,
    );

    let net = NetworkInitializer::initialize_network_with_implementation(
        "config.toml",
        drone_factories,
        client_factories,
        server_factories,
    );

    loop_forever_sc(net)
}
