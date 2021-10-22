// jkcoxson

use core::panic;
use std::env;
use std::process::Command;

fn main() {
    match env::consts::OS {
        "windows" => todo!("Windows support is WIP"),
        "macos" => {
            // Detect if brew is installed
            if !env::var("HOMEBREW_PREFIX").is_ok() {
                println!("Homebrew is not installed, installing it now");
                // If not, install it
                Command::new("/bin/bash")
                    .arg("-c")
                    .arg("\"$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)\"")
                    .status()
                    .expect("Failed to install Homebrew, aborting");
            }
            println!("Homebrew is installed, fetching dependencies");
            // Install openssl if not already installed
            Command::new("/bin/bash")
                .arg("-c")
                .arg("\"$(brew install openssl)\"")
                .status()
                .expect("Failed to install openssl, aborting");

            // Install pkg-config if not already installed
            Command::new("/bin/bash")
                .arg("-c")
                .arg("\"$(brew install pkg-config)\"")
                .status()
                .expect("Failed to install pkg-config, aborting");
            unix_build();
        }
        "linux" => {
            // Get linux distribution as a string
            let distro = Command::new("lsb_release")
                .arg("-is")
                .output()
                .expect("Failed to get linux distribution, aborting");
            let distro = String::from_utf8(distro.stdout)
                .expect("Failed to get linux distribution, aborting");
            // Detect package manager
            let pkg_manager = get_package_manager(distro.to_ascii_lowercase());
            println!("Detected {} package manager", pkg_manager);
            // Install git
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install git)\"", pkg_manager))
                .status()
                .expect("Failed to install git, aborting");
            // Install build-essentials
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!(
                    "\"$(sudo {} install build-essential)\"",
                    pkg_manager
                ))
                .status()
                .expect("Failed to install build-essentials, aborting");
            // Install glibtool
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install glibtool)\"", pkg_manager))
                .status()
                .expect("Failed to install glibtool, aborting");
            // Install make
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install make)\"", pkg_manager))
                .status()
                .expect("Failed to install make, aborting");
            // Install cpp
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install cpp)\"", pkg_manager))
                .status()
                .expect("Failed to install cpp, aborting");
            // Install gcc-8
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install gcc-8)\"", pkg_manager))
                .status()
                .expect("Failed to install gcc-8, aborting");
            // Install clang
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install clang)\"", pkg_manager))
                .status()
                .expect("Failed to install clang, aborting");
            // Install checkinstall
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install checkinstall)\"", pkg_manager))
                .status()
                .expect("Failed to install checkinstall, aborting");
            // Intall autoconf
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install autoconf)\"", pkg_manager))
                .status()
                .expect("Failed to install autoconf, aborting");
            // Install automake
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install automake)\"", pkg_manager))
                .status()
                .expect("Failed to install automake, aborting");
            // Install libtool
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install libtool)\"", pkg_manager))
                .status()
                .expect("Failed to install libtool, aborting");
            // Install m4
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install m4)\"", pkg_manager))
                .status()
                .expect("Failed to install m4, aborting");
            // Install python-dev
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install python-dev)\"", pkg_manager))
                .status()
                .expect("Failed to install python-dev, aborting");
            // Install pkg-config
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install pkg-config)\"", pkg_manager))
                .status()
                .expect("Failed to install pkg-config, aborting");
            // Install libavahi-client-dev
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!(
                    "\"$(sudo {} install libavahi-client-dev)\"",
                    pkg_manager
                ))
                .status()
                .expect("Failed to install libavahi-client-dev, aborting");
            // Install cython
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install cython)\"", pkg_manager))
                .status()
                .expect("Failed to install cython, aborting");
            // Install autoheader
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install autoheader)\"", pkg_manager))
                .status()
                .expect("Failed to install autoheader, aborting");
            // Install libusb-1.0-0-dev
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!(
                    "\"$(sudo {} install libusb-1.0-0-dev)\"",
                    pkg_manager
                ))
                .status()
                .expect("Failed to install libusb-1.0-0-dev, aborting");
            // Install libssl-dev
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install libssl-dev)\"", pkg_manager))
                .status()
                .expect("Failed to install libssl-dev, aborting");
            // Install libc6-udeb
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install libc6-udeb)\"", pkg_manager))
                .status()
                .expect("Failed to install libc6-udeb, aborting");
            // Install libc6-dev
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install libc6-dev)\"", pkg_manager))
                .status()
                .expect("Failed to install libc6-dev, aborting");
            // Install libtool-bin
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install libtool-bin)\"", pkg_manager))
                .status()
                .expect("Failed to install libtool-bin, aborting");
            // Install libplist++-dev
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!(
                    "\"$(sudo {} install libplist++-dev)\"",
                    pkg_manager
                ))
                .status()
                .expect("Failed to install libplist++-dev, aborting");
            // Install libplist++
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install libplist++)\"", pkg_manager))
                .status()
                .expect("Failed to install libplist++, aborting");
            // Install openssl
            Command::new("/bin/bash")
                .arg("-c")
                .arg(format!("\"$(sudo {} install openssl)\"", pkg_manager))
                .status()
                .expect("Failed to install openssl, aborting");
            unix_build();
        }
        _ => panic!("Unsupported operating system"),
    }
}

/// Builds libimobiledevice after the dependencies have been installed
fn unix_build() {
    // Create libimobiledevice folder at $HOME
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(mkdir -p $HOME/libimobiledevice)\"")
        .status()
        .expect("Failed to create libimobiledevice folder, aborting");
    // Change directory to ~/libimobiledevice
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(cd $HOME/libimobiledevice)\"")
        .status()
        .expect("Failed to change directory to libimobiledevice, aborting");
    // Clone libplist
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(git clone https://github.com/libimobiledevice/libplist.git\"")
        .status()
        .expect("Failed to clone libplist, aborting");
    // Change directory to ~/libimobiledevice/libplist
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(cd $HOME/libimobiledevice/libplist)\"")
        .status()
        .expect("Failed to change directory to libplist, aborting");
    // Run autogen
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(./autogen.sh)\"")
        .status()
        .expect("Failed to run autogen, aborting");
    // Make
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(make)\"")
        .status()
        .expect("Failed to make libplist, aborting");
    // Install
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(sudo make install)\"")
        .status()
        .expect("Failed to install libplist, aborting");
    // Exit directory
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(cd ..)\"")
        .status()
        .expect("Failed to exit libplist directory, aborting");
    // Clone libusbmuxd
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(git clone https://github.com/libimobiledevice/libusbmuxd.git\"")
        .status()
        .expect("Failed to clone libusbmuxd, aborting");
    // Change directory to ~/libimobiledevice/libusbmuxd
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(cd $HOME/libimobiledevice/libusbmuxd)\"")
        .status()
        .expect("Failed to change directory to libusbmuxd, aborting");
    // Run autogen
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(./autogen.sh)\"")
        .status()
        .expect("Failed to run autogen, aborting");
    // Make
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(make)\"")
        .status()
        .expect("Failed to make libusbmuxd, aborting");
    // Install
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(sudo make install)\"")
        .status()
        .expect("Failed to install libusbmuxd, aborting");
    // Exit directory
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(cd ..)\"")
        .status()
        .expect("Failed to exit libusbmuxd directory, aborting");
    // Clone libimobiledevice
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(git clone https://github.com/libimobiledevice/libimobiledevice.git\"")
        .status()
        .expect("Failed to clone libimobiledevice, aborting");
    // Change directory to libimobiledevice
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(cd libimobiledevice)\"")
        .status()
        .expect("Failed to change directory to libimobiledevice, aborting");
    // Run autogen.sh
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(./autogen.sh)\"")
        .status()
        .expect("Failed to run autogen.sh, aborting");
    // Build libimobiledevice
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(make)\"")
        .status()
        .expect("Failed to build libimobiledevice, aborting");
    // Install libimobiledevice
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(sudo make install)\"")
        .status()
        .expect("Failed to install libimobiledevice, aborting");
    // Exit the directory
    Command::new("/bin/bash")
        .arg("-c")
        .arg("\"$(cd ..)\"")
        .status()
        .expect("Failed to exit libimobiledevice, aborting");
    println!("libimobiledevice is installed");
}

/// Gets the package manager from the name of the linux Distribution
fn get_package_manager(os: String) -> String {
    match os.trim() {
        "debian" => "apt",
        "ubuntu" => "apt",
        "centos" => "yum",
        "fedora" => "yum",
        "arch" => "pacman",
        _ => panic!("Unsupported operating system"),
    }
    .to_string()
}
