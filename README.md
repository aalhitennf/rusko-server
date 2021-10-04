Server of the [Rusko](https://github.com/aalhitennf/rusko-client) remote app. Rusko allows you to run commands, control your mouse, send key presses and transfer files remotely from your mobile device. 

**Usage**

    rusko [FLAGS]

    FLAGS:
        -h, --help       Prints help information
        -v, --verbose    Enable console logging.
        -V, --version    Prints version information

 **Installation**  

 Arch Linux:  

     yay -S rusko-server

On other distros, after building, move the binary manually where you want, i.e. `mv build/rusko-v0.1.1/rusko /usr/bin/rusko`.

**Building**

Install rust and cargo.

- Linux
    - `./build.sh`  
- Windows
    - `cargo build --release`

Building in windows should be ok with `x86_64-pc-windows-msvc` toolchain

**Runtime dependecies**

- Linux
    - xdotool


**Build dependencies**

- Linux
    - libxdo-dev
- Windows
    - `x86_64-pc-windows-msvc` toolchain
    - [C++ Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)
 
**Configuration**

Rusko server uses config files from `$XDG_CONFIG_HOME/rusko/` and creates them if not found. Default `config.toml` will look like this:

    port = "6551"
    password = ""

Set the password and you're good to go. **NOTICE: PASSWORD MUST BE EXACTLY 16 CHARACTERS LONG** for encryption to work properly.

**Optional values**

By default Rusko will use home folder to save transferred files. You can set optional destination folder in `config.toml` with: 

    upload_folder = "/path/to/uploads"

**Commands**

You can define commands that client is allowed to run, in file `commands`, located in `$XDG_CONFIG_HOME/rusko/commands`. I included few useful commands in the defaults for controlling pulseaudio.

Syntax for commands is `alias :: command`, one per line.

Example:  

    Example :: notify-send "Hello" "This is example command"

Rusko monitors this file for changes, so there's no need to restart whole server for adding and removing commands.

**Paired devices**

Device ids are stored in `$XDG_CONFIG_HOME/rusko/paired_devices`, formatted `deviceid:device_name`. You can pair device only once.

**Notice**:
Unpairing from client, when there's no connection between the client and server, results on you having paired device in this file, and not being able to pair again from that client. You need to manually remove your device from this file.

Rusko monitors this file for changes, so there's no need to restart for the changes.

**Security**

Almost all traffic between client and server is secured with AES-128 bit encryption, only exception being mouse movement and presses. Encrypting those makes no point, they're just numbers that carry no sensitive information.


**Disclaimer**

Rusko was made mainly for personal use and learning purposes, no guarantees or warranties.
