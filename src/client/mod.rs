use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
    execute,
    terminal::{Clear, ClearType},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{
    io::{self, BufRead, BufReader, Read, Write},
    net::TcpStream,
    sync::mpsc,
    thread,
    time::Duration,
};

use std::io::stdout;

pub fn start_client(ip: &str) -> io::Result<()> {
    let mut stream = TcpStream::connect(format!("{}:6767", ip))?; // me conecto

    println!("Escribe tu nombre de usuario:");
    let mut username = String::new();
    io::stdin().read_line(&mut username)?; // para conseguir el username?
    let username = username.trim().to_string();

    stream.write_all(username.as_bytes())?;
    stream.write_all(b"\n")?;

    enable_raw_mode()?; // para entrar al modo crudo
    execute!(stdout(), Clear(ClearType::All))?; // limpiar para evitar pequeños errorsitos que
                                                // pasaron
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel::<String>();

    let mut read_stream = stream.try_clone()?;
    thread::spawn(move || {
        let mut reader = BufReader::new(&mut read_stream);
        let mut line = String::new();

        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    let msg = line.trim_end().to_string();
                    if !msg.is_empty() {
                        let _ = tx.send(msg);
                    }
                }
                Err(_) => break,
            }
        }
    });

    let result = run_ui(&mut terminal, &mut stream, rx);

    disable_raw_mode()?;
    terminal.show_cursor()?;

    result
}

fn run_ui(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    stream: &mut TcpStream,
    rx: mpsc::Receiver<String>,
) -> io::Result<()> {
    let mut input = String::new();
    let mut messages: Vec<String> = vec!["bienvenido al chat uwu".to_string()]; // mensaje inicial
                                                                                // de bienvenida
                                                                                // hacia el bello y
                                                                                // amado usuario

    loop {
        while let Ok(msg) = rx.try_recv() {
            messages.push(msg);
            if messages.len() > 200 {
                let excess = messages.len() - 200;
                messages.drain(0..excess);
            }
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(3)])
                .split(f.size());

            let chat_text = messages.join("\n");
            let chat = Paragraph::new(chat_text)
                .block(Block::default().borders(Borders::ALL).title("Chat")); // sin boprdes
                                                                              // creamos una cajita
                                                                              // llamada chat

            let input_box = Paragraph::new(input.as_str())
                .block(Block::default().borders(Borders::ALL).title("Escribe aqui abajo")); // y
                                                                                            // otra
                                                                                            // cajita
                                                                                            // para
                                                                                            // manejar
                                                                                            // la
                                                                                            // entrada
                                                                                            // del
                                                                                            // usuario

            f.render_widget(chat, chunks[0]); // renderizar el primero
            f.render_widget(input_box, chunks[1]); // y ahora con el segundo
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => input.push(c),
                    KeyCode::Backspace => { // borrar un pedazo
                        input.pop();
                    }
                    KeyCode::Enter => { // enviar mensajes
                        let msg = input.trim().to_string();
                        if !msg.is_empty() {
                            stream.write_all(msg.as_bytes())?;
                            stream.write_all(b"\n")?;
                        }
                        input.clear();
                    }
                    KeyCode::Esc => break, // adriana salte
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
