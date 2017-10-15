use std::env;
use std::fs::{OpenOptions, File};
use std::io::ErrorKind;
use std::io::Write;
use std::ops::Add;
use std::process::Command;

// Package cache can be kept around for 12 hours.
const MAX_CACHE_AGE: u64 = 12 * 60 * 60;

fn main() {
    let path = build_cache_path();
    if let Some(mut file) = cache_reload_required(path.as_ref()) {
        update_cache(&mut file)
    }

    // All args passed to nixfd should be passed to ripgrep (they're assumed to be rg flags)
    let mut args: Vec<String> = env::args().skip(1).collect();
    args.push(path);

    Command::new("rg")
        .args(args)
        .spawn().expect("Could not start ripgrep");
}

fn build_cache_path() -> String {
    let cache_key = "XDG_CACHE_HOME";
    //  $XDG_CACHE_HOME defines the base directory relative to which user specific non-essential
    // data files should be stored. If  is either not set or empty, a default equal
    // to $HOME/.cache should be used.

    let cache_dir = match env::var(cache_key) {
        Ok(cache_dir) => cache_dir,
        _ => {
            let home = env::var("HOME").expect("Home folder not set");
            home.add("/.cache")
        }
    };

    format!("{}/nixpkgs", cache_dir)
}

fn update_cache(file: &mut File) {
    println!("Updating nix package cache");
    let pkgs = load_nix_pkgs();
    file.write(pkgs.as_bytes()).expect("Updating cache failed");
}

/// Checks whether the nix package cache is older than 24 hours and needs to be rebuilt.
/// Returns a mutable file handle if an update is required.
fn cache_reload_required(path: &str) -> Option<File> {
    let opened = OpenOptions::new()
        .write(true).create_new(false)
        .open(path.to_string());

    let file = match opened {
        Ok(f) => f,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                File::create(path.to_string()).expect("Could not create cache file")
            } else {
                panic!("Could not open cache file '{}': {}", path, e)
            }
        }
    };

    let modified = file.metadata()
        .expect("Could not read file metadata")
        .modified().unwrap();

    let elapsed = modified.elapsed().unwrap().as_secs();

    if elapsed > MAX_CACHE_AGE {
        Some(file)
    } else {
        None
    }
}

/// Query nix-env for its package (or "derivation") list
fn load_nix_pkgs() -> String {
    let output = Command::new("nix-env")
        .args(&["-qaP"])
        .output()
        .expect("Failed to query Nix database");

    String::from_utf8(output.stdout).expect("Invalid nix-env output")
}
