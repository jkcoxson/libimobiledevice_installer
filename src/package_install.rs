use std::process::Command;

// jkcoxson

/// Uses brew to install a package.
pub fn brew(package: &str) {
    println!("Installing {} with brew", package);
    Command::new("/bin/bash")
        .arg("-c")
        .arg(format!("\"$(brew install {})\"", package))
        .status()
        .expect(format!("Failed to install {}, aborting", package).as_str());
}

/// Uses the specified package manager to install the specified package.
pub fn linux(pkg_manager: String, package: &str) {
    println!("Installing {} with {}", package, pkg_manager);
    Command::new("/bin/bash")
        .arg("-c")
        .arg(format!("\"$(sudo {} install {})\"", pkg_manager, package))
        .status()
        .expect(format!("Failed to install {}, aborting", package).as_str());
}
