use url::form_urlencoded::byte_serialize;
use crate::Config;
use super::rpt_client;

pub enum Command {
    Up(u8), //u
    Down(u8), //d
    Left(u8), //l
    Right(u8), //r
    Select(u8), //s
    Back(u8), //b
    Home, //h
    Play, //play
    Rev(u8), //rev
    Fwd(u8), //fwd
    InstantReplay(u8), //rw
    Info, //i
    Backspace(u8), //x
    VolumeUp(u8), //v+
    VolumeDown(u8), //v-
    VolumeMute, //vx
    PowerOff, //o
    Keyboard(String), //k
}
impl Command {
    pub fn parse(raw:&str) -> Option<Self> {
        use Command::*;
        let cm;
        let mut rpt:Option<u8> = None;
        let mut keys:Option<&str> = None;
        if raw.contains(":") {
            (cm,keys) = (&raw[0..1],Some(&raw[1..]))
        } else {
            (cm,rpt) = split(raw)?;
        };
        Some(match cm {
            "u" | "up" => Up(rpt?),
            "d" | "down" => Down(rpt?),
            "l" | "left" | "<" => Left(rpt?),
            "r" | "right" | ">" => Right(rpt?),
            "s" | "select" => Select(rpt?),
            "b" | "back" => Back(rpt?),
            "h" | "home" => Home,
            "p" | "play" => Play,
            "rev" | "reverse" => Rev(rpt?),
            "fwd" | "forward" => Fwd(rpt?),
            "rw" | "replay" | "rewind" => InstantReplay(rpt?),
            "i" | "info" | "*" => Info,
            "x" | "backspace" => Backspace(rpt?),
            "+" | "v+" | "volume+" => VolumeUp(rpt?),
            "-" | "v-" | "volume-" => VolumeDown(rpt?),
            "m" | "mute" | "vx" => VolumeMute,
            "o" | "off" | "power" | "poweroff" => PowerOff,
            ":" => Keyboard(keys?.to_string()),
            _ => return None,
        })
    }
    pub fn run(&self,config:&Config) {
        use Command::*;
        let post = rpt_client(&config.addr);
        match self {
            Up(rpt) => post("keypress/Up",*rpt,50),
            Down(rpt) => post("keypress/Down",*rpt,50),
            Left(rpt) => post("keypress/Left",*rpt,50),
            Right(rpt) => post("keypress/Right",*rpt,50),
            Select(rpt) => post("keypress/Select",*rpt,50),
            Back(rpt) => post("keypress/Back",*rpt,250),
            Home => post("keypress/Home",1,250),
            Play => post("keypress/Play",1,250),
            Rev(rpt) => post("keypress/Rev",*rpt,250),
            Fwd(rpt) => post("keypress/Fwd",*rpt,250),
            InstantReplay(rpt) => post("keypress/InstantReplay",*rpt,50),
            Info => post("keypress/Info",1,100),
            Backspace(rpt) => post("keypress/Backspace",*rpt,50),
            VolumeUp(rpt) => post("keypress/VolumeUp",*rpt,50),
            VolumeDown(rpt) => post("keypress/VolumeDown",*rpt,50),
            VolumeMute => post("keypress/VolumeMute",1,50),
            PowerOff => post("keypress/PowerOff",1,2000),
            Keyboard(keys) => {
                for substr in keys.split("") {
                    let encoded:String = byte_serialize(substr.as_bytes()).collect();
                    post(&format!("keypress/Lit_{encoded}"),1,50)
                }
            },
        }
    }
}

fn split(raw:&str) -> Option<(&str,Option<u8>)> {
    let mut first_num:Option<usize> = None;
    for (i,c) in raw.chars().enumerate() {
        if c.is_numeric() {
            first_num = Some(i);
            break;
        }
    }
    Some(match first_num {
        Some(i) => {
            let mut rpt = raw[i..].parse::<u8>().ok()?;
            if rpt == 0 {rpt = 1}
            (&raw[..i],Some(rpt))
        },
        None => (raw,Some(1)),
    })
}
