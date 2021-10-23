// jkcoxson

use core::panic;
use std::env;
use std::process::Command;
mod package_install;
use package_install::{brew, linux};

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
            brew("openssl");
            // Install pkg-config if not already installed
            brew("pkg-config");
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
            linux(pkg_manager.clone(), "git");
            // Install build-essentials
            linux(pkg_manager.clone(), "build-essential");
            // Install glibtool
            linux(pkg_manager.clone(), "glibtool");
            // Install make
            linux(pkg_manager.clone(), "make");
            // Install cpp
            linux(pkg_manager.clone(), "cpp");
            // Install gcc-8
            linux(pkg_manager.clone(), "gcc-8");
            // Install clang
            linux(pkg_manager.clone(), "clang");
            // Install checkinstall
            linux(pkg_manager.clone(), "checkinstall");
            // Intall autoconf
            linux(pkg_manager.clone(), "autoconf");
            // Install automake
            linux(pkg_manager.clone(), "automake");
            // Install libtool
            linux(pkg_manager.clone(), "libtool");
            // Install m4
            linux(pkg_manager.clone(), "m4");
            // Install python-dev
            linux(pkg_manager.clone(), "python-dev");
            // Install pkg-config
            linux(pkg_manager.clone(), "pkg-config");
            // Install libavahi-client-dev
            linux(pkg_manager.clone(), "libavahi-client-dev");
            // Install cython
            linux(pkg_manager.clone(), "cython");
            // Install autoheader
            linux(pkg_manager.clone(), "autoheader");
            // Install libusb-1.0-0-dev
            linux(pkg_manager.clone(), "libusb-1.0-0-dev");
            // Install libssl-dev
            linux(pkg_manager.clone(), "libssl-dev");
            // Install libc6-udeb
            linux(pkg_manager.clone(), "libc6-udeb");
            // Install libc6-dev
            linux(pkg_manager.clone(), "libc6-dev");
            // Install libtool-bin
            linux(pkg_manager.clone(), "libtool-bin");
            // Install libplist++-dev
            linux(pkg_manager.clone(), "libplist++-dev");
            // Install libplist++
            linux(pkg_manager.clone(), "libplist++");
            // Install openssl
            linux(pkg_manager.clone(), "openssl");
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
