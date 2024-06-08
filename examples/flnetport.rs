#![forbid(unsafe_code)]

use {
    flemish::{
        app,
        button::Button,
        color_themes,
        enums::FrameType,
        frame::Frame,
        group::Flex,
        input::{Input, InputType},
        prelude::*,
        valuator::{Counter, CounterType},
        OnEvent, Sandbox, Settings,
    },
    std::{
        collections::HashMap,
        net::{SocketAddr, TcpStream},
        thread,
        time::Duration,
    },
};

pub fn main() {
    Model::new().run(Settings {
        size: (640, 360),
        resizable: false,
        ignore_esc_close: true,
        color_map: Some(color_themes::DARK_THEME),
        scheme: Some(app::Scheme::Base),
        ..Default::default()
    })
}

#[derive(Clone, Copy)]
enum Message {
    Octet(usize, u8),
    Port(u32),
    Check,
}

#[derive(Clone)]
struct Model {
    status: String,
    address: [u8; 4],
    port: u32,
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Self {
            status: "Status".to_string(),
            address: [127, 0, 0, 1],
            port: 22,
        }
    }

    fn title(&self) -> String {
        format!(
            "[{}] {:?}:{} - FlNetPort",
            self.status, self.address, self.port
        )
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill().column();
        {
            let mut header = Flex::default();
            header.fixed(&Frame::default().with_label("IP address:"), WIDTH);
            for idx in 0..4 {
                crate::counter(self.address[idx] as f64, &mut header)
                    .on_event(move |octet| Message::Octet(idx, octet.value() as u8));
            }
            Frame::default();
            crate::input(&self.port.to_string(), &mut header)
                .with_label("Port:")
                .on_event(move |input| Message::Port(input.value().parse::<u32>().unwrap()));
            header.fixed(
                &Button::default()
                    .with_label("@#->")
                    .clone()
                    .on_event(move |_| Message::Check),
                HEIGHT,
            );
            header.end();
            header.set_pad(0);
            page.fixed(&header, HEIGHT);
        }
        Frame::default()
            .with_label(&self.status)
            .set_frame(FrameType::DownFrame);
        page.end();
        page.set_pad(PAD);
        page.set_margin(PAD);
        page.set_frame(FrameType::FlatBox);
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Octet(idx, octet) => self.address[idx] = octet,
            Message::Port(port) => self.port = port,
            Message::Check => {
                let address: String = self
                    .address
                    .iter()
                    .map(|octet| octet.to_string())
                    .collect::<Vec<String>>()
                    .join(".")
                    + &format!(":{}", self.port);

                if address.parse::<SocketAddr>().is_ok() {
                    self.status = "Scanning...".to_string();
                    let services = HashMap::from(SERVICES);
                    if services.contains_key(&self.port) {
                        app::first_window()
                            .unwrap()
                            .set_label(&format!("{} - FlNetPort", services[&self.port]));
                    }
                    let handler = thread::spawn(move || -> bool {
                        TcpStream::connect_timeout(
                            &address.parse::<SocketAddr>().unwrap(),
                            Duration::from_secs(8),
                        )
                        .is_ok()
                    });
                    if let Ok(check) = handler.join() {
                        self.status = match check {
                            true => "Status: Open",
                            false => "Status: Closed",
                        }
                        .to_string();
                    }
                } else {
                    self.status = "Invalid IP/Port".to_string();
                }
            }
        }
    }
}

fn counter(value: f64, flex: &mut Flex) -> Counter {
    let mut element = Counter::default().with_type(CounterType::Simple);
    element.set_maximum(254_f64);
    element.set_precision(0);
    element.set_value(value);
    flex.fixed(&element, WIDTH);
    element
}

fn input(value: &str, flex: &mut Flex) -> Input {
    let mut element = Input::default().with_type(InputType::Int);
    element.set_value(value);
    flex.fixed(&element, WIDTH);
    element
}

const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WIDTH: i32 = HEIGHT * 3;
const SERVICES: [(u32, &str); 31] = [
    (21, "FTP"),
    (990, "FTPS"),
    (22, "SSH"),
    (23, "Telnet"),
    (53, "DNS"),
    (25, "SMTP"),
    (587, "SMTP (SSL)"),
    (110, "POP"),
    (995, "POP (SSL)"),
    (143, "IMAP"),
    (993, "IMAP (SSL)"),
    (67, "DHCP"),
    (123, "NTP"),
    (80, "HTTP"),
    (8080, "HTTP"),
    (443, "HTTPS"),
    (194, "IRC"),
    (445, "SMB"),
    (5060, "SIP"),
    (3306, "MySQL"),
    (5432, "PostgreSQL"),
    (27017, "MongoDB"),
    (6379, "Redis"),
    (2082, "cPanel"),
    (6000, "X11"),
    (5672, "AMQP"),
    (389, "LDAP"),
    (636, "LDAPS"),
    (9987, "TeamSpeak 3"),
    (666, "Doom"),
    (25565, "Minecraft"),
];
