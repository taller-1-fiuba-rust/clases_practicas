use eframe::egui;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Host Port Checker",
        options,
        Box::new(|_cc| Box::new(HostCheckerApp::default())),
    )
}

struct AppConfig {
    host: String,
    ports: String,
    use_multiple_threads: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            ports: "80,443,8080".to_string(),
            use_multiple_threads: true,
        }
    }
}

struct HostCheckerApp {
    config: AppConfig,
    results: Vec<(u16, bool)>,
    open_ports: Vec<u16>,
    is_scanning: bool,
    progress: f32,
    total_ports: usize,
    rx: Option<Receiver<(u16, bool)>>,
}

impl Default for HostCheckerApp {
    fn default() -> Self {
        Self {
            config: AppConfig::default(),
            results: Vec::new(),
            open_ports: Vec::new(),
            is_scanning: false,
            progress: 0.0,
            total_ports: 0,
            rx: None,
        }
    }
}

impl eframe::App for HostCheckerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Host Port Checker");

            // lee host
            ui.horizontal(|ui| {
                ui.label("Host:");
                ui.text_edit_singleline(&mut self.config.host);
            });

            // lee puertos
            ui.horizontal(|ui| {
                ui.label("Ports (comma-separated):");
                ui.text_edit_singleline(&mut self.config.ports);
            });

            // lee si debe usar multiples threads
            ui.checkbox(
                &mut self.config.use_multiple_threads,
                "Use multiple threads",
            );

            ui.horizontal(|ui| {
                let scan_btn_label = if self.is_scanning {
                    "Scanning..."
                } else {
                    "Scan Ports"
                };

                // boton para escanear lista de puertos
                if ui.button(scan_btn_label).clicked() && !self.is_scanning {
                    self.start_scan(false);
                }

                // boton para scanear todos los puertos
                if ui.button("Scan All Ports").clicked() && !self.is_scanning {
                    self.start_scan(true);
                }
            });

            // Lee channel y actualiza resultados
            if self.is_scanning {
                while let Ok((port, is_open)) = self.rx.as_mut().unwrap().try_recv() {
                    self.results.push((port, is_open));
                    if is_open {
                        self.open_ports.push(port);
                    }
                }

                self.progress = self.results.len() as f32 / self.total_ports as f32;

                // actualiza barra de progreso
                ui.add(egui::ProgressBar::new(self.progress).show_percentage());

                // condicion de corte
                if self.results.len() == self.total_ports {
                    self.is_scanning = false;
                    self.open_ports.sort_unstable();
                }
            }

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("All Scanned Ports:");
                    egui::ScrollArea::vertical()
                        .id_source("all_ports")
                        .max_height(200.0)
                        .show(ui, |ui| {
                            for (port, is_open) in &self.results {
                                let text = format!(
                                    "Port {}: {}",
                                    port,
                                    if *is_open { "Open" } else { "Closed" }
                                );
                                ui.label(text);
                            }
                        });
                });

                ui.vertical(|ui| {
                    ui.label("Open Ports:");
                    egui::ScrollArea::vertical()
                        .id_source("open ports")
                        .max_height(200.0)
                        .show(ui, |ui| {
                            for &port in &self.open_ports {
                                ui.colored_label(
                                    egui::Color32::GREEN,
                                    format!("Port {}: Open", port),
                                );
                            }
                        });
                });
            });
        });

        // Request a repaint on each frame while scanning
        if self.is_scanning {
            ctx.request_repaint();
        }
    }
}

impl HostCheckerApp {
    fn start_scan(&mut self, all_ports: bool) {
        self.is_scanning = true;
        self.results.clear();
        self.open_ports.clear();
        self.progress = 0.0;
        let host = self.config.host.clone();
        let ports: Vec<u16> = if all_ports {
            (1..=65535).collect()
        } else {
            self.config
                .ports
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect()
        };
        self.total_ports = ports.len();

        let (tx, rx) = channel();
        self.rx = Some(rx);

        if self.config.use_multiple_threads {
            let num_threads = num_cpus::get().min(ports.len());
            let chunk_size = (ports.len() + num_threads - 1) / num_threads;

            for chunk in ports.chunks(chunk_size) {
                let host = host.clone();
                let chunk = chunk.to_vec();
                let tx = tx.clone();

                thread::spawn(move || {
                    for &port in &chunk {
                        let is_open = check_port(&host, port);
                        tx.send((port, is_open)).unwrap();
                    }
                });
            }
        } else {
            thread::spawn(move || {
                for &port in &ports {
                    let is_open = check_port(&host, port);
                    tx.send((port, is_open)).unwrap();
                }
            });
        }
    }
}

fn check_port(host: &str, port: u16) -> bool {
    let ip = IpAddr::from_str(host).unwrap_or_else(|_| {
        // default: localhost
        IpAddr::from([127, 0, 0, 1])
    });
    let socket = SocketAddr::new(ip, port);
    TcpStream::connect_timeout(&socket, Duration::from_millis(200)).is_ok()
}
