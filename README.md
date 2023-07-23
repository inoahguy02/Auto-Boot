# Auto-Boot

Auto boot manager desktop app to be created using Rust. Will be originally created for Windows being the main system and Linux being on the external drive

# Works Partially (Work in progress)
As of the latest push/release, the app has is very limited. The app can be found in target\debug\auto-boot.exe. It will require administrator privelages to run.<br>
The only function that exists right now is booting in to Ubuntu Linux from Windows whenever the executable is ran.

# Goals (Most important first)

1. When connecting an external drive with an OS on it, the app will detect it and will ask the user if they will like to boot in to the OS
2. Implement an option for the user to manually tell the app to boot in to the OS
3. Add cross platform support for MacOS and Linux
4. Add support for internal drives and partitions
5. Add support for MacOS Bootcamp?
