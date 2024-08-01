use gtk::{glib, prelude::*, Label, Switch};
use gtk::{Application, ApplicationWindow, Box, Orientation, ToggleButton};
use libappindicator::{AppIndicator, AppIndicatorStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::{Child, Command};
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Tunnel {
    name: String,
    socks_port: u16,
    ssh_host: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    tunnels: Vec<Tunnel>,
}

fn main() {
    let config_path = format!(
        "{}/.config/ssh_tunnel_manager/config.yaml",
        std::env::var("HOME").unwrap()
    );
    let config_str = fs::read_to_string(config_path).unwrap();
    let config: Config = serde_yaml::from_str(&config_str).unwrap();

    let application = Application::new(
        Some("network.apparatus.ssh_tunnel_manager"),
        Default::default(),
    );

    let tunnels = Arc::new(Mutex::new(config.tunnels));
    let processes = Arc::new(Mutex::new(HashMap::new()));
    let app_ref = application.clone();

    application.connect_activate(move |app| {
        let window = ApplicationWindow::new(app);
        window.set_title("SSH Tunnel Manager");
        window.set_default_size(350, 70);

        let outer_container = Box::new(Orientation::Vertical, 0);
        outer_container.set_margin(20); // Add padding around the entire content

        let container = Box::new(Orientation::Vertical, 10);

        let tunnels_clone = Arc::clone(&tunnels);
        let processes_clone = Arc::clone(&processes);

        for tunnel in tunnels_clone.lock().unwrap().iter() {
            let row = Box::new(Orientation::Horizontal, 10);

            let label = Label::new(Some(&tunnel.name));
            label.set_halign(gtk::Align::Start);
            label.set_hexpand(true);

            let switch = Switch::new();
            switch.set_halign(gtk::Align::End);

            row.add(&label);
            row.add(&switch);

            let tunnel_clone = tunnel.clone();
            let processes_clone = Arc::clone(&processes_clone);

            switch.connect_active_notify(move |switch| {
                let mut processes = processes_clone.lock().unwrap();
                if switch.is_active() {
                    // Start the tunnel
                    match start_tunnel(&tunnel_clone) {
                        Ok(child) => {
                            processes.insert(tunnel_clone.name.clone(), child);
                            println!("Started tunnel: {}", tunnel_clone.name);
                        }
                        Err(e) => {
                            eprintln!("Failed to start tunnel {}: {}", tunnel_clone.name, e);
                            switch.set_active(false);
                        }
                    }
                } else {
                    // Stop the tunnel
                    if let Some(mut child) = processes.remove(&tunnel_clone.name) {
                        match child.kill() {
                            Ok(_) => println!("Stopped tunnel: {}", tunnel_clone.name),
                            Err(e) => {
                                eprintln!("Failed to stop tunnel {}: {}", tunnel_clone.name, e)
                            }
                        }
                    }
                }
            });

            container.add(&row);
        }

        outer_container.add(&container);
        window.add(&outer_container);

        // Create tray icon
        let mut indicator = AppIndicator::new("ssh-tunnel-manager", "network-vpn");
        indicator.set_status(AppIndicatorStatus::Active);

        let mut menu = gtk::Menu::new();
        let show_item = gtk::MenuItem::with_label("Show");
        let quit_item = gtk::MenuItem::with_label("Quit");

        menu.append(&show_item);
        menu.append(&quit_item);
        menu.show_all();

        indicator.set_menu(&mut menu);

        let window_clone = window.clone();
        show_item.connect_activate(move |_| {
            window_clone.show();
            window_clone.present();
        });

        let app_ref_clone = app_ref.clone();
        quit_item.connect_activate(move |_| {
            app_ref_clone.quit();
        });

        window.connect_delete_event(move |window, _| {
            window.hide();
            glib::Propagation::Stop
        });

        window.show_all();
    });

    application.run();
}

fn start_tunnel(tunnel: &Tunnel) -> Result<Child, std::io::Error> {
    Command::new("ssh")
        .arg("-D")
        .arg(tunnel.socks_port.to_string())
        .arg("-q")
        .arg("-C")
        .arg("-N")
        .arg("-f")
        .arg(&tunnel.ssh_host)
        .spawn()
}
