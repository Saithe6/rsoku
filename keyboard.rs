use std::{time::Duration,collections::HashMap};
use crossterm::{event::{self,DisableBracketedPaste,EnableBracketedPaste,Event,KeyCode,KeyEvent,KeyModifiers},execute,terminal};
use url::form_urlencoded::byte_serialize;
use serde::{Serialize,Deserialize};
use crate::{start_client,Config};

pub fn kbd(config:&Config) {
    let post = start_client(&config.addr);
    let mut countdown:u8 = 0;
    init();
    loop {
        if event::poll(Duration::from_millis(5)).is_ok_and(|x| x) {
            // lets us read events while waiting for input delay to end
            // this way events don't build up
            if countdown > 0 {
                let _ = event::read();
            } else {
                match event::read() {
                    Ok(Event::Key(kev)) => {
                        if kev.code == KeyCode::Char('c') && kev.modifiers == KeyModifiers::CONTROL {
                            break;
                        }
                        if let Some(action) = KeyAndMods::from_key_event(kev).action(&config.keybinds) {
                            // run the action and match on its ExitInstr
                            // in most cases, if the action was successful, this will be Nothing
                            match action.run(&post,&mut countdown) {
                                ExitInstr::Nothing => (),
                                ExitInstr::Break => break,
                                ExitInstr::Error => {
                                    cleanup();
                                    panic!("panicked on ExitInstr")
                                },
                            }
                        }
                    },
                    Err(err) => {
                        cleanup();
                        panic!("{err}")
                    },
                    _ => (),
                }
            }
        }
        countdown = countdown.saturating_sub(1);
    }
    cleanup();
}

#[derive(Debug,Hash,PartialEq,Eq,Serialize,Deserialize)]
pub struct KeyAndMods(pub KeyCode,pub u8);
impl KeyAndMods {
    fn from_key_event(kev:KeyEvent) -> Self {
        KeyAndMods(kev.code,kev.modifiers.bits())
    }
}

pub type KeyMap = HashMap<KeyAndMods,Action>;

#[derive(Serialize,Deserialize,Clone,Copy,Debug,PartialEq,Eq)]
pub enum Action {
    Exit,
    Up,
    Down,
    Left,
    Right,
    Select,
    Home,
    Back,
    Play,
    Rev,
    Fwd,
    InstantReplay,
    VolumeUp,
    VolumeDown,
    VolumeMute,
    Info,
    PowerOff,
    KeyboardInput,
    // this action exists for configuration purposes
    Nothing,
}
impl Action {
    fn run(&self,post:&impl Fn(&str),countdown:&mut u8) -> ExitInstr {
        self.run_custom_delay(post,self.get_delay(),countdown)
    }
    fn run_custom_delay(&self,post:&impl Fn(&str),delay:u8,countdown:&mut u8) -> ExitInstr {
        macro_rules! action {
            ($post:expr) => {
                post($post);
                *countdown = delay;
            };
        }
        match self {
            Self::Nothing => (),
            Self::Exit => return ExitInstr::Break,
            Self::Up => {action!("keypress/Up");},
            Self::Down => {action!("keypress/Down");},
            Self::Left => {action!("keypress/Left");},
            Self::Right => {action!("keypress/Right");},
            Self::Select => {action!("keypress/Select");},
            Self::Home => {action!("keypress/Home");},
            Self::Back => {action!("keypress/Back");},
            Self::Play => {action!("keypress/Play");},
            Self::Rev => {action!("keypress/Rev");},
            Self::Fwd => {action!("keypress/Fwd");},
            Self::InstantReplay => {action!("keypress/InstantReplay");},
            Self::VolumeUp => {action!("keypress/VolumeUp");},
            Self::VolumeDown => {action!("keypress/VolumeDown");},
            Self::VolumeMute => {action!("keypress/VolumeMute");},
            Self::Info => {action!("keypress/Info");},
            Self::PowerOff => {action!("keypress/PowerOff");},
            Self::KeyboardInput => loop {
                match event::read() {
                    Ok(Event::Key(kev)) => match kev.code {
                        KeyCode::Char(ch) if kev.modifiers == KeyModifiers::NONE || kev.modifiers == KeyModifiers::SHIFT
                            => {
                                let encoded:String = byte_serialize(ch.to_string().as_bytes()).collect();
                                post(&format!("keypress/Lit_{encoded}"));
                                *countdown = delay;
                            },
                        KeyCode::Backspace if kev.modifiers == KeyModifiers::NONE
                            => {
                                post("keypress/Backspace");
                                *countdown = delay;
                            },
                        KeyCode::Esc => break,
                        _ => (),
                    },
                    Ok(Event::Paste(text)) => {
                        for substr in text.split("") {
                            let encoded:String = byte_serialize(substr.as_bytes()).collect();
                            post(&format!("keypress/Lit_{encoded}"));
                            *countdown = delay;
                        }
                    },
                    Err(..) => return ExitInstr::Error,
                    _ => (),
                }
            }
        };
        ExitInstr::Nothing
    }
    // Delays are in increments of 5 milliseconds, because our event::poll timeout is 5 milliseconds
    fn get_delay(&self) -> u8 {
        match self {
            Self::Nothing | Self::Exit | Self::KeyboardInput
                => 0,
            Self::Up | Self::Down | Self::Left | Self::Right |
            Self::Select | Self::Home | Self::Back | Self::Info |
            Self::Play| Self::Rev | Self::Fwd | Self::InstantReplay
                => 10,
            Self::VolumeUp | Self::VolumeDown | Self::VolumeMute | Self::PowerOff
                => 50,
        }
    }
}
enum ExitInstr {
    Break,
    Error,
    Nothing,
}

trait ActionMap {
    fn action(&self,keybinds:&KeyMap) -> Option<Action>;
}
impl ActionMap for KeyAndMods {
    fn action(&self,keybinds:&KeyMap) -> Option<Action> {
        keybinds.get(self).copied()
    }
}

fn init() {
    let _ = terminal::enable_raw_mode();
    execute!(
        std::io::stdout(),
        EnableBracketedPaste
    ).unwrap();
}

fn cleanup() -> Option<()> {
    let _ = terminal::disable_raw_mode();
    fn rec_commands(i:u16) -> Option<()> {
        if i > 1000 {return None;}
        let res = execute!(
            std::io::stdout(),
            DisableBracketedPaste
        );
        if res.is_err() {rec_commands(i+1)}
        else {res.ok()}
    }
    rec_commands(0)
}
