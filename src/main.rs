use std::process::Command;
use regex::Regex;
use std::io::{self, Write};

fn get_os() -> String {
    print!("Type the OS to boot in to: ");
    io::stdout().flush().unwrap();
    let mut op_sys = String::new();
    io::stdin().read_line(&mut op_sys).unwrap();
    op_sys.trim().to_string()
}

fn main() {
    //Get GUID for target OS
    if cfg!(target_os = "windows") {
        let output = Command::new("cmd")
            .arg("/C bcdedit /enum firmware")
            .output()
            .expect("Failed to execute");

        let guid;
        let con_output = String::from_utf8_lossy(&output.stdout).to_string();
        loop {
            //Use regex to parse OS's section, then parse the GUID
            //Rust doesn't support lookarounds in its Regex which is dumb
            let section = Regex::new(&format!(r"identifier.+(?:\n.+){{3}}{}", get_os().to_lowercase())).unwrap();

            if let Some(capture) = section.captures(&con_output) {
                let guid_pattern = Regex::new(r"\{\S+\}").unwrap();
                guid = guid_pattern.find(capture.get(0).unwrap().as_str()).unwrap().as_str();
                println!("Capture: {}", guid);
                break;
            }
            else {
                println!("No capture found. Make sure spelling is correct and the operating system has been booted in to before.\n");
            };
        }

        //Use GUID to set Ubuntu as default boot
        Command::new("cmd")
            .arg(format!("/C bcdedit /set {{fwbootmgr}} default {}", guid))
            .output()
            .expect("Failed to execute");
        
        //Restart machine
        Command::new("cmd")
            .arg("/C shutdown /r /t 0")
            .output()
            .expect("Failed to execute");

        //Keep app running
        //std::io::stdin().read_line(&mut String::new()).unwrap();
    }
    else if cfg!(target_os = "linux") {
        //This will ask user for password
        let output = Command::new("sudo")
            .arg("efibootmgr")
            .output()
            .expect("Failed to execute");

        let con_output = String::from_utf8_lossy(&output.stdout).to_string();
        let boot_num;
        loop {
            let line = Regex::new(&format!(r"(?i)\d{{4}}\* {}", get_os())).unwrap();
            if let Some(capture) = line.captures(&con_output) {
                let guid_pattern = Regex::new(r"\d{4}").unwrap();
                boot_num = guid_pattern.find(capture.get(0).unwrap().as_str()).unwrap().as_str();
                println!("Capture: {}", boot_num);
                break;
            }
            else {
                println!("No capture found. Make sure spelling is correct and the operating system has been booted in to before.\n");
            };
        }

        //Set Windows as default boot
        Command::new("sudo")
            .args(["efibootmgr", "-o", boot_num])
            .output()
            .expect("Failed to execute");
        
        //Restart machine
        Command::new("sudo")
            .arg("reboot")
            .output()
            .expect("Failed to execute");
    }
    /*else if cfg!(target_os = "macOS") {
        //Don't have mac so idk when this will be a thing
        //Can prob find a cheap one on ebay or something idk
    }*/
    else {
        //Can't find host OS
        return;
    }
}
