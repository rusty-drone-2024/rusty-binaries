#![warn(clippy::pedantic)]
use client::Client as ChatClient;
use client_tui::loop_forever_chat_tui;
use client_ui::loop_forever_media_gui;
use common_structs::leaf::Leaf;
use matteo_contribution as mc;
use network_initializer::factory::DroneFactory;
use network_initializer::factory::DroneImpl;
use network_initializer::factory::DroneRunnable;
use network_initializer::factory::LeafFactory;
use network_initializer::factory::LeafImpl;
use network_initializer::factory::LeafRunnable;
use network_initializer::{drone_factories, leaf_factories, NetworkInitializer};
//use rusty_drones::RustyDrone;
use ap2024_unitn_cppenjoyers_drone::CppEnjoyersDrone;
use bagel_bomber::BagelBomber;
use d_r_o_n_e_drone::MyDrone as DRONEDrone;
use dr_ones::Drone as DrOnes;
use fungi_drone::FungiDrone;
use lockheedrustin_drone::LockheedRustin;
use rustafarian_drone::RustafarianDrone;
use rustbusters_drone::RustBustersDrone;
use rusty_drones_servers::{ChatServer, MediaServer, TextServer};
use simulation_controller::loop_forever_sc;
use std::env;
use wg_2024::drone::Drone;
use wg_2024_rust::drone::RustDrone;
use LeDron_James::Drone as LeDronJames;

fn main() {
    let bin = env::args().nth(1);
    let extra = env::args().nth(2);

    match bin {
        Some(bin) if bin == "media-gui" => {
            loop_forever_media_gui(extra.expect("An address was not passed"));
        }
        Some(bin) if bin == "chat-tui" => {
            let _ = loop_forever_chat_tui(extra.expect("A port was not passed"));
        }
        _ => start_simulation(),
    }
}

fn start_simulation() {
    //let drone_factories = drone_factories!(RustyDrone, "RustyDrones");
    let drone_factories = drone_factories!(
        RustafarianDrone,
        "Rustafarian",
        DrOnes,
        "Dr-Ones",
        FungiDrone,
        "Fungi",
        DRONEDrone,
        "D-R-O-N-E",
        CppEnjoyersDrone,
        "Cpp Enjoyers",
        LockheedRustin,
        "Lockheed Rustin",
        LeDronJames,
        "LeDron James",
        BagelBomber,
        "Bagel Bomber",
        RustDrone,
        "Rust",
        RustBustersDrone,
        "Rust Busters",
    );

    let client_factories = leaf_factories!(
        mc::TextMediaClient,
        "Di Noia Media",
        ChatClient,
        "Casarotto Chat",
        ChatClient,
        "Casarotto Chat",
    );
    let server_factories = leaf_factories!(
        mc::TextServer,
        "Di Noia Text",
        mc::MediaServer,
        "Di Noia Text",
        TextServer,
        "Mens Text",
        MediaServer,
        "Mens Media",
        ChatServer,
        "Mens Chat",
    );

    let net = NetworkInitializer::initialize_network_with_implementation(
        "config.toml",
        drone_factories,
        client_factories,
        server_factories,
    );

    loop_forever_sc(net);
}
