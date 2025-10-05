use std::io::Write;

use discord_rich_presence::DiscordIpc;
use macroquad::prelude::*;
pub struct Game<'a> {
    /// Directory where the save is stored
    pub save_dir: String,
    /// Game data
    pub data: Data,
    /// Game settings
    pub settings: Settings,
    pub devices: Vec<Device>,
    pub current_screen: Screens,
    pub previous_screen: Option<Screens>,
    pub fonts: Vec<Font>,
    pub audio: Audio,
    pub cursor: Cursor,
    /// Discord Rich Presence
    pub drp: DrpClient<'a>,
}

impl<'a> Game<'a> {
    pub async fn init() -> Self {
        Self {
            save_dir: "./data/saves/".to_string(),
            data: Data::init(),
            settings: Settings::init(),
            devices: Vec::new(),
            current_screen: Screens::MainMenu,
            previous_screen: None,
            fonts: vec![
                load_ttf_font_from_bytes(include_bytes!(
                    "../assets/fonts/ProFont/ProFontIIxNerdFont-Regular.ttf"
                ))
                .unwrap(),
                load_ttf_font_from_bytes(include_bytes!(
                    "../assets/fonts/Terminus/TerminessNerdFont-Regular.ttf"
                ))
                .unwrap(),
            ],
            audio: Audio::init(),
            cursor: Cursor::new().await,
            drp: DrpClient::new(),
        }
    }

    pub fn create_game_file(&self) {
        let path = format!("{}{}.save", &self.save_dir, &self.data.player.name);
        let _file = std::fs::File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&path)
            .expect(&format!("File at \"{}\" could not be opened", path));
    }

    pub fn save_game(&self) {
        let path = format!("{}{}.save", &self.save_dir, &self.data.player.name);
        let mut file = std::fs::File::options()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&path)
            .expect(&format!("File at \"{}\" could not be opened", path));
        let data =
            serde_json::to_string_pretty(&serde_json::json!([&self.data, &self.settings])).unwrap();
        _ = file.write(data.as_bytes());
    }

    pub fn load_game(&mut self, name: &str) {
        let path = format!("{}{}.save", &self.save_dir, name);
        let file = std::fs::File::options()
            .read(true)
            .open(&path)
            .expect(&format!("File at \"{}\" could not be opened", path));
        let reader = std::io::BufReader::new(file);

        let objects: Vec<serde_json::Value> = serde_json::from_reader(reader).unwrap();

        self.data = serde_json::from_value(objects[0].clone()).unwrap();
        self.settings = serde_json::from_value(objects[1].clone()).unwrap();
    }
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Data {
    pub player: Player,
    pub bits: u64,
    pub bytes: u64,
    pub bytestrings: u64,
    pub money: f64,
}

impl Data {
    pub fn init() -> Self {
        Self {
            player: Player::new(),
            bits: 0,
            bytes: 0,
            bytestrings: 0,
            money: 60.0,
        }
    }
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
/// Contains game settings like volume, difficulty, etc.
pub struct Settings {
    /// Musuc volume
    pub mus_vol: f32,
    /// Sound effects volume
    pub sfx_vol: f32,
    /// Fine-tunable difficulty level (0-255)
    pub difficulty: u8,
    /// Whether Discord Rich Presence is enabled or not
    pub discord_rich_presence: bool,
}

impl Settings {
    pub fn init() -> Self {
        Self {
            mus_vol: 0.5,
            sfx_vol: 0.5,
            difficulty: 122,
            discord_rich_presence: true,
        }
    }
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
/// Basic Device base class used for all functional equipmentin the game
pub struct Device {
    /// This Device's name
    pub name: String,
    /// This Device's type (e.g. Miner)
    pub device_type: DeviceType,
    /// This Device's efficiency
    pub efficiency: u8,
    /// The devices unique ID
    pub id: u8,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
/// The type of a device (e.g. Miner)
pub enum DeviceType {
    /// Mines Bits
    Miner,
    /// Converts 8 Bits to 1 Byte
    Converter,
    /// Assembles Bytes into ByteStrings
    Assembler,
    /// Analyzes ByteStrings, showing their content as UTF-8 characters
    Analyzer,
    /// Can extract certain things from ByteStrings
    Extractor,
}

#[derive(Debug, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum Screens {
    MainMenu,
    SaveMenu,
    SettingsMenu,
    InGame,
    PauseMenu,
    DeviceManagement,
    GameOver,
}

pub struct Audio {
    pub _stream: rotilities::OutputStream,
    pub stream_handle: rotilities::OutputStreamHandle,
    pub music_sinks: Vec<rotilities::Sink>,
    pub sfx_sinks: Vec<rotilities::Sink>,
}

impl Audio {
    pub fn init() -> Self {
        let (stream, stream_handle) = rotilities::init();
        Self {
            _stream: stream,
            stream_handle: stream_handle,
            music_sinks: Vec::new(),
            sfx_sinks: Vec::new(),
        }
    }
}

pub enum Alignment {
    Left,
    Center,
    Right,
}

pub struct Cursor {
    pub x: f32,
    pub y: f32,
    pub sprite: Texture2D,
    pub sprite_hover: Texture2D,
    pub sprite_click: Texture2D,
    pub hovers_clickable: bool,
}

impl Cursor {
    pub async fn new() -> Self {
        let (x, y) = mouse_position();
        Self {
            x,
            y,
            sprite: load_texture("./assets/sprites/Cursor.png")
                .await
                .expect("Failed to load cursor sprite"),
            sprite_hover: load_texture("./assets/sprites/Cursor_hover.png")
                .await
                .expect("Failed to load cursor hover sprite"),
            sprite_click: load_texture("./assets/sprites/Cursor_click.png")
                .await
                .expect("Failed to load cursor click sprite"),
            hovers_clickable: false,
        }
    }

    pub fn update(&mut self) {
        let (x, y) = mouse_position();
        match self.hovers_clickable {
            true => {
                if is_mouse_button_down(MouseButton::Left) {
                    draw_texture(
                        &self.sprite_click,
                        x - self.sprite_click.width() / 2.0,
                        y - self.sprite_click.height() / 2.0,
                        WHITE,
                    );
                } else {
                    draw_texture(
                        &self.sprite_hover,
                        x - self.sprite_hover.width() / 2.0,
                        y - self.sprite_hover.height() / 2.0,
                        WHITE,
                    );
                }
            }
            false => {
                draw_texture(
                    &self.sprite,
                    x - self.sprite.width() / 2.0,
                    y - self.sprite.height() / 2.0,
                    WHITE,
                );
            }
        }
        self.x = x;
        self.y = y;
    }
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Player {
    pub name: String,
    pub age: u16,
    pub location: String,
    pub domain: Domain,
}

impl Player {
    pub fn new() -> Self {
        Self {
            name: "Player".to_string(),
            age: 18,
            location: "".to_string(),
            domain: Domain::init(),
        }
    }
}

pub struct DrpClient<'a> {
    client: discord_rich_presence::DiscordIpcClient,
    activity: discord_rich_presence::activity::Activity<'a>,
    previous_state: &'a str
}

impl<'a> DrpClient<'a> {
    pub fn new() -> Self {
        Self {
            client: discord_rich_presence::DiscordIpcClient::new("1424335981036834887").unwrap(),
            activity: discord_rich_presence::activity::Activity::new(),
            previous_state: ""
        }
    }

    pub fn update(&mut self, state: Option<&'a str>, details: &'a str) {
        if state.is_some() {
            self.previous_state = state.unwrap();
        }
        self.activity = match state {
            Some(c) => discord_rich_presence::activity::Activity::new()
                    .details(details)
                    .state(state.unwrap()),
            None => discord_rich_presence::activity::Activity::new()
                .details(details)
                .state(self.previous_state)
        };
        _ = self.client.set_activity(self.activity.clone());
    }

    pub fn connect(&mut self, sink: &rotilities::Sink) {
        match self.client.connect() {
            Ok(_) => {}
            Err(_) => rotilities::play_audio(sink, "./assets/sound/sfx/fail.mp3"),
        };
    }
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Domain {
    current: Domains,
    pub name: String,
}

impl Domain {
    pub fn init() -> Self {
        Self {
            current: Domains::Prime,
            name: "Domain [Prime]".to_string()
        }
    }

    pub fn set_current(&mut self, domain: Domains) {
        self.name = match domain {
            Domains::Prime => "Domain [Prime]".to_string(),
            Domains::Expanse => "Domain [Expanse]".to_string(),
            Domains::Void => "Domain [Void]".to_string(),
            Domains::Redacted => "[Redacted]".to_string(),
            Domains::D56XFG => "Domain [D56XFG]".to_string(),
        };
        self.current = domain;
    }

    pub fn name(&self) -> String {
        match &self.current {
            Domains::Prime => "Domain [Prime]".to_string(),
            Domains::Expanse => "Domain [Expanse]".to_string(),
            Domains::Void => "Domain [Void]".to_string(),
            Domains::Redacted => "[Redacted]".to_string(),
            Domains::D56XFG => "Domain [D56XFG]".to_string(),
        }
    }
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum Domains {
    Prime,
    Expanse,
    Void,
    Redacted,
    D56XFG,
}
