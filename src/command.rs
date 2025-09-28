use url::form_urlencoded::byte_serialize;
use crate::Config;
use super::rpt_client;

pub enum Command {
    Up(u8),
    Down(u8),
    Left(u8),
    Right(u8),
    Select(u8),
    Back(u8),
    Home,
    Pause,
    Rev(u8),
    Fwd(u8),
    InstantReplay(u8),
    Info,
    Backspace(u8),
    VolumeUp(u8),
    VolumeDown(u8),
    VolumeMute,
    PowerOff,
    Keyboard(String),
    ChannelUp(u8),
    ChannelDown(u8),
    InputHDMI1,
    InputHDMI2,
    InputHDMI3,
    InputHDMI4,
    InputAV1,
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
            "p" | "pause" => Pause,
            "rev" | "reverse" => Rev(rpt?),
            "fwd" | "forward" => Fwd(rpt?),
            "rw" | "replay" | "rewind" => InstantReplay(rpt?),
            "i" | "info" | "*" => Info,
            "x" | "backspace" => Backspace(rpt?),
            "+" | "v+" | "volume+" => VolumeUp(rpt?),
            "-" | "v-" | "volume-" => VolumeDown(rpt?),
            "m" | "mute" | "vx" => VolumeMute,
            "o" | "off" | "power" | "poweroff" => PowerOff,
            "chu" | "channelup" => ChannelUp(rpt?),
            "chd" | "channeldown" => ChannelDown(rpt?),
            "hdmi" if rpt.is_some_and(|x| x == 1) => InputHDMI1,
            "hdmi" if rpt.is_some_and(|x| x == 2) => InputHDMI2,
            "hdmi" if rpt.is_some_and(|x| x == 3) => InputHDMI3,
            "hdmi" if rpt.is_some_and(|x| x == 4) => InputHDMI4,
            "av" => InputAV1,
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
            Pause => post("keypress/Play",1,250),
            Rev(rpt) => post("keypress/Rev",*rpt,250),
            Fwd(rpt) => post("keypress/Fwd",*rpt,250),
            InstantReplay(rpt) => post("keypress/InstantReplay",*rpt,50),
            Info => post("keypress/Info",1,100),
            Backspace(rpt) => post("keypress/Backspace",*rpt,50),
            VolumeUp(rpt) => post("keypress/VolumeUp",*rpt,50),
            VolumeDown(rpt) => post("keypress/VolumeDown",*rpt,50),
            VolumeMute => post("keypress/VolumeMute",1,50),
            PowerOff => post("keypress/PowerOff",1,2000),
            ChannelUp(rpt) => post("keypress/ChannelUp",*rpt,250),
            ChannelDown(rpt) => post("keypress/ChannelDown",*rpt,250),
            InputHDMI1 => post("keypress/InputHDMI1",1,250),
            InputHDMI2 => post("keypress/InputHDMI2",1,250),
            InputHDMI3 => post("keypress/InputHDMI3",1,250),
            InputHDMI4 => post("keypress/InputHDMI4",1,250),
            InputAV1 => post("keypress/InputAV1",1,250),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        Command::parse("hdmi1").unwrap();
        Command::parse("hdmi2").unwrap();
        Command::parse("hdmi3").unwrap();
        Command::parse("hdmi4").unwrap();
    }
}
