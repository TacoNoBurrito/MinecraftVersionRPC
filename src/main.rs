extern crate discord_rpc_client;
use discord_rpc_client::Client;
use std::{thread, time, process, process::Command};

struct Data {
    version: String,
}

fn main() {
    println!("Process Started. PID: {}", process::id());
    Command::new("cmd")
    .args(&["/C", "start", "minecraft://"]);
    let version = Command::new("powershell").args(&["/C", "Get-AppxPackage -name Microsoft.MinecraftUWP | select -expandproperty Version"]).output();
    let mut data = Data {
        version: format!("{}", "nan"),
    };
    match version {
        Ok(o) => {
            unsafe {
                data.version = String::from_utf8_unchecked(o.stdout).to_string();
            }
        }
        Err(e) => {
            println!("Err {}", e);
        }
    }
    let mut drpc = Client::new(879449275015241798);
    drpc.start();
    let ver = data.version.trim();
    loop {
        drpc.set_activity(|act| act.state(format!("Version: {}", ver)))
        .expect("Failed to update rpc");
        thread::sleep(time::Duration::from_secs(10));
    }
}
