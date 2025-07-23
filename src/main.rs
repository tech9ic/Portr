use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Direction},
    style::{Style, Color},
    widgets::{Block, Borders, Row, Table},
    Terminal,
};
use std::{error::Error, io, time::{Duration, Instant}, process::Command, fs};
use sysinfo::{System, Pid};

struct ConnectionInfo {
    proto: String,
    local_ip: String,
    local_port: String,
    remote_ip: String,
    remote_port: String,
    pid: String,
    process_name: String,
    row_style: Style,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut selected: usize = 0;
    let mut message: Option<(String, Instant)> = None;
    loop {
        // Fetch live network data
        let data = get_network_data();
        let row_count = data.len();
        if selected >= row_count && row_count > 0 {
            selected = row_count - 1;
        }
        let rows: Vec<Row> = data.iter().enumerate().map(|(i, info)| {
            let mut row = Row::new(vec![
                info.proto.clone(),
                info.local_ip.clone(),
                info.local_port.clone(),
                info.remote_ip.clone(),
                info.remote_port.clone(),
                info.pid.clone(),
                info.process_name.clone(),
            ]).style(info.row_style);
            if i == selected {
                row = row.style(info.row_style.bg(Color::Blue));
            }
            row
        }).collect();

        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("Portr by NeRDs").borders(Borders::ALL);
            let header = ["Proto", "Local IP", "LPort", "Remote IP", "RPort", "PID", "Process"];
            let table = Table::new(rows)
                .header(Row::new(header).style(Style::default().fg(Color::Yellow)))
                .block(block)
                .widths(&[
                    Constraint::Length(6),
                    Constraint::Length(15),
                    Constraint::Length(6),
                    Constraint::Length(15),
                    Constraint::Length(6),
                    Constraint::Length(8),
                    Constraint::Length(16),
                ]);
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
                .split(size);
            f.render_widget(table, chunks[0]);
            // Draw confirmation message or permanent footer
            let area = chunks[1];
            let footer_text = if let Some((ref msg, ref when)) = message {
                if when.elapsed() < Duration::from_secs(2) {
                    Some((msg.clone(), Style::default().fg(Color::Cyan)))
                } else {
                    None
                }
            } else {
                None
            };
            if let Some((text, style)) = footer_text {
                let paragraph = ratatui::widgets::Paragraph::new(text).style(style);
                f.render_widget(paragraph, area);
            } else {
                let paragraph = ratatui::widgets::Paragraph::new("Press 'q' or Esc to quit").style(Style::default().fg(Color::DarkGray));
                f.render_widget(paragraph, area);
            }
        })?;
        // Handle input
        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Down => {
                        if selected + 1 < row_count {
                            selected += 1;
                        }
                    }
                    KeyCode::Up => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyCode::Char('b') => {
                        if let Some((remote_ip, remote_port)) = get_selected_remote(&data, selected) {
                            let added = add_to_list_file("blacklist.txt", &remote_ip, &remote_port);
                            message = Some((if added { format!("Added {} or {} to blacklist", remote_ip, remote_port) } else { "Already in blacklist".to_string() }, Instant::now()));
                        }
                    }
                    KeyCode::Char('w') => {
                        if let Some((remote_ip, remote_port)) = get_selected_remote(&data, selected) {
                            let added = add_to_list_file("whitelist.txt", &remote_ip, &remote_port);
                            message = Some((if added { format!("Added {} or {} to whitelist", remote_ip, remote_port) } else { "Already in whitelist".to_string() }, Instant::now()));
                        }
                    }
                    _ => {}
                }
            }
        }
        // Redraw every 2 seconds
        if last_tick.elapsed() >= Duration::from_secs(2) {
            last_tick = Instant::now();
        }
    }
    Ok(())
}

fn get_network_data() -> Vec<ConnectionInfo> {
    let mut data = Vec::new();
    let whitelist = read_list_file("whitelist.txt");
    let blacklist = read_list_file("blacklist.txt");
    let suspicious_ports = vec![
        "6667", "31337", "12345", "54321", "4444", "5555", "23", "2323", "1337", "31331", "31338", "31339", "31340", "31341", "31342", "31343", "31344", "31345", "31346", "31347", "31348", "31349", "31350"
    ];
    let output = Command::new("netstat")
        .args(&["-ano"])
        .output();
    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut system = System::new_all();
        system.refresh_all();
        for line in stdout.lines() {
            let line = line.trim_start();
            if line.starts_with("TCP") || line.starts_with("UDP") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 5 {
                    let proto = parts[0];
                    let local = parts[1];
                    let remote = parts[2];
                    let pid_str = parts[4];
                    let pid = pid_str.parse::<u32>().unwrap_or(0);
                    let (local_ip, local_port) = split_ip_port(local);
                    let (remote_ip, remote_port) = split_ip_port(remote);
                    let process_name = system
                        .process(Pid::from_u32(pid))
                        .map(|p| p.name().to_string_lossy().to_string())
                        .unwrap_or_else(|| "-".to_string());
                    let mut row_style = Style::default();
                    if blacklist.contains(&remote_ip.to_string()) || blacklist.contains(&remote_port.to_string()) {
                        row_style = Style::default().fg(Color::Red);
                    } else if whitelist.contains(&remote_ip.to_string()) || whitelist.contains(&remote_port.to_string()) {
                        row_style = Style::default().fg(Color::Green);
                    } else if suspicious_ports.contains(&remote_port) {
                        row_style = Style::default().fg(Color::Yellow);
                    }
                    data.push(ConnectionInfo {
                        proto: proto.to_string(),
                        local_ip: local_ip.to_string(),
                        local_port: local_port.to_string(),
                        remote_ip: remote_ip.to_string(),
                        remote_port: remote_port.to_string(),
                        pid: pid.to_string(),
                        process_name,
                        row_style,
                    });
                }
            }
        }
    }
    data
}

fn read_list_file(filename: &str) -> Vec<String> {
    match fs::read_to_string(filename) {
        Ok(contents) => contents
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .collect(),
        Err(_) => Vec::new(),
    }
}

// Helper function to split "IP:PORT" or "[IPv6]:PORT"
fn split_ip_port(addr: &str) -> (&str, &str) {
    if let Some(idx) = addr.rfind(':') {
        let (ip, port) = addr.split_at(idx);
        let ip = ip.trim_start_matches('[').trim_end_matches(']');
        let port = &port[1..];
        (ip, port)
    } else {
        (addr, "-")
    }
}

// Helper to get the remote IP and port of the selected row
fn get_selected_remote(data: &[ConnectionInfo], selected: usize) -> Option<(String, String)> {
    if selected < data.len() {
        Some((data[selected].remote_ip.clone(), data[selected].remote_port.clone()))
    } else {
        None
    }
}

// Helper to add to list file (avoiding duplicates)
fn add_to_list_file(filename: &str, ip: &str, port: &str) -> bool {
    let mut list = read_list_file(filename);
    let mut changed = false;
    if !list.contains(&ip.to_string()) {
        list.push(ip.to_string());
        changed = true;
    }
    if !list.contains(&port.to_string()) {
        list.push(port.to_string());
        changed = true;
    }
    if changed {
        let contents = list.join("\n");
        let _ = fs::write(filename, contents);
    }
    changed
}
