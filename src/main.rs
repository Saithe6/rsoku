use std::{env,fs,thread::sleep,time::Duration};
use reqwest::blocking;
use serde::{Serialize,Deserialize};
mod command;
use command::Command;
mod keyboard;
use keyboard::KeyMap;

fn main() {
    let configdir = match env::var("XDG_CONFIG_HOME") {
        Ok(val) => format!("{val}/rsoku/config.ron"),
        Err(..) => match env::var("HOME") {
            Ok(val) => format!("{val}/rsoku.ron"),
            Err(..) => match env::var("USER") {
                Ok(val) => format!("/home/{val}/.config/rsoku/config.ron"),
                Err(..) => panic!("cannot find config file: HOME, USER, and XDG_CONFIG_HOME are all unset"),
            }
        },
    };
    let config:Config = match fs::read_to_string(&configdir) {
        Ok(val) => match ron::from_str(&val) {
            Ok(val) => val,
            Err(err) => panic!("{0} at line: {1} collum: {2}",err.code,err.position.line,err.position.col)
        },
        Err(..) => panic!("cannot find config file at {configdir}"),
    };
    let mut argiter = env::args().peekable();
    argiter.next();
    if argiter.peek().is_none() {
        keyboard::kbd(&config)
    } else {
        for arg in argiter {
            match Command::parse(&arg) {
                Some(cm) => cm.run(&config),
                None => panic!("invalid input")
            }
        }
    }
}

#[derive(Serialize,Deserialize,Debug)]
struct Config {
    pub addr:String,
    pub keybinds:KeyMap,
}

pub fn rpt_client(addr:&str) -> impl Fn(&str,u8,u64) + use<'_> {
    let post = start_client(addr);
    move |command:&str,rpt:u8,delay:u64| {
        for _ in 0..rpt {
            post(command);
            sleep(Duration::from_millis(delay))
        }
    }
}

pub fn start_client(addr:&str) -> impl Fn(&str) + use<'_> {
    let client = blocking::Client::new();
    move |command:&str| {
        let _ = client.post(format!("http://{addr}:8060/{command}")).send();
    }
}
