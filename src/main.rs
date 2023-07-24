use std::process::Command;
use regex::Regex;

fn main() {
    /*Not going to implement auto usb plug in listener just yet
    Will go ahead and implement Windows -> Linux using cmd
    Probably ask user for distro

    Run cmd as admin
    Use command: bcdedit /enum firmware
    Search for OS (e.x. Ubuntu) and grab identifier string
    Use command: bcdedit /set {fwbootmgr} default {identifier} (Replace identifier with grabbed string)
    Use command: shutdown /r /t 0 (Make sure user knows that the system will be shut down and to save everything)
    Should boot straight in to OS*/

    //For windows. Change to sh for linux
    
    //Get GUID for target OS
    if cfg!(target_os = "windows")
    {
        let output = Command::new("cmd")
            .arg("/C bcdedit /enum firmware")
            .output()
            .expect("Failed to execute");

        //Use regex to parse OS's section, then parse the GUID
        //Rust doesn't support lookarounds in its Regex which is dumb
        let con_output = String::from_utf8_lossy(&output.stdout).to_string();
        let section = Regex::new(r"identifier.+(?:\n.+){3}ubuntu").unwrap();
        let guid;

        if let Some(capture) = section.captures(&con_output)
        {
            let guid_pattern = Regex::new(r"\{\S+\}").unwrap();
            guid = guid_pattern.find(capture.get(0).unwrap().as_str()).unwrap().as_str();
            println!("Capture: {}", guid);
        }
        else
        {
            println!("No capture found");
            return;
        };

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
    else if cfg!(target_os = "linux")
    {
        //This will ask user for password
        let output = Command::new("sudo")
            .arg("efibootmgr")
            .output()
            .expect("Failed to execute");

        let con_output = String::from_utf8_lossy(&output.stdout).to_string();
        let line = Regex::new(r"\d{4}\* Windows").unwrap();
        let boot_num;

        if let Some(capture) = line.captures(&con_output)
        {
            let guid_pattern = Regex::new(r"\d{4}").unwrap();
            boot_num = guid_pattern.find(capture.get(0).unwrap().as_str()).unwrap().as_str();
            println!("Capture: {}", boot_num);
        }
        else
        {
            println!("No capture found: {}", String::from_utf8_lossy(&output.stderr).to_string());
            return;
        };

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
    /*else if cfg!(target_os = "macOS")
    {
        //Don't have mac so idk when this will be a thing
        //Can prob find a cheap one on ebay or something idk
    }*/
    else
    {
        //Can't find OS
        return;
    }
    
    
}
