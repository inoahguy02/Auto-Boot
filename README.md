# Auto-Boot

Auto boot manager desktop app created using Rust. 

## Note
When entering a Linux distribution, list the parent/forked distribution and not the flavor<br>
Ex: If booting into Mint, type Ubuntu. If booting into Ubuntu, type Ubuntu

# Todo:

1. Research and implement a way to boot different flavors of a distro. Ex: Mint, Kubuntu, Lubuntu
2. Add support for MacOS
3. Turn into background process that will run on startup. Will have an event listener for the external drive being plugged in
4. Move out of the terminal stage and implement app with UI to select drive and OS
