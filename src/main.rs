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
    let output = if cfg!(target_os = "windows")
    {
        Command::new("cmd")
            .args(["/C", "bcdedit /enum firmware"])
            .output()
            .expect("Failed to execute")
    }
    else
    {
        return;
    };
    
    let con_output = String::from_utf8_lossy(&output.stdout).to_string();
    let section = Regex::new(r"identifier.+(?:\n.+){3}ubuntu").unwrap();

    if let Some(capture) = section.captures(&con_output)
    {
        let guid_pattern = Regex::new(r"\{\S+\}").unwrap();
        let guid = guid_pattern.find(capture.get(0).unwrap().as_str()).unwrap().as_str();
        println!("Capture: {}", guid);
    }
    else
    {
        println!("No capture found");
    };


    std::io::stdin().read_line(&mut String::new()).unwrap();
}
