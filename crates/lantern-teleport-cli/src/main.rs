//! CLI tool to teleport to Lantern locations in Bloodborne save files
//!
//! Usage: lantern-teleport <save_file> [--location <LOCATION>]

use clap::Parser;
use lantern_teleport_core::{get_all_locations, search_locations, teleport, Location};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "lantern-teleport",
    author,
    version,
    about = "A CLI tool to teleport to any Lantern in Bloodborne save files",
    long_about = "A CLI tool to teleport to any Lantern in Bloodborne save files.\n\n\
                  This tool is meant to be run on userdata0000, userdata0001, etc. files \
                  found in your Bloodborne save directory.\
                  userdata0000 is your first character, userdata0001 is your second character, and so on."
)]
struct Args {
    /// Path to the save file (e.g., userdata0000, userdata0001)
    #[arg( num_args = 0..=1)]
    save_file: Option<PathBuf>,

    /// Teleport to a specific location (supports fuzzy matching)
    #[arg(short, long)]
    location: Option<String>,

    /// List all available locations
    #[arg(long)]
    list: bool,
}

/// Display all available locations grouped by region
fn list_locations() {
    let locations = get_all_locations();

    // Group by region
    let mut regions: Vec<(&str, Vec<&Location>)> = Vec::new();
    for loc in locations {
        if let Some(existing) = regions.iter_mut().find(|(name, _)| *name == loc.region) {
            existing.1.push(loc);
        } else {
            regions.push((loc.region, vec![loc]));
        }
    }

    let region_count = regions.len();

    println!("\nAvailable teleport locations:");
    println!("============================");

    for (region, locs) in regions {
        println!("\n{}", region);
        println!("----------------------------");
        for location in locs {
            println!(
                "  - {} (X: {:.2}, Y: {:.2}, Z: {:.2})",
                location.name, location.x, location.y, location.z
            );
        }
    }

    println!("\n============================");
    println!(
        "Total: {} locations across {} regions",
        locations.len(),
        region_count
    );
}

/// Handle search results and return the location if exactly one match
/// Otherwise, prints error and exits
fn handle_search_results(query: &str, matches: Vec<&'static Location>) -> &'static Location {
    if matches.is_empty() {
        println!(
            "Error: No locations found matching '{}'\nUse --list to see available locations",
            query
        );
        std::process::exit(1);
    }

    if matches.len() > 1 {
        println!("Multiple matches found for '{}':\n", query);
        for (i, loc) in matches.iter().enumerate() {
            println!("  {}. {} ({})", i + 1, loc.name, loc.region);
        }
        println!("\nPlease provide a more specific location name.");
        std::process::exit(1);
    }

    // Exactly one match
    let location = matches[0];
    println!(
        "Found location: {} (X: {:.2}, Y: {:.2}, Z: {:.2})",
        location.name, location.x, location.y, location.z
    );
    location
}

fn main() {
    let args = Args::parse();

    if args.list {
        list_locations();
        return;
    }

    let Some(save_file) = args.save_file else {
        println!("Error: No save file provided\nUse --help for usage information");
        std::process::exit(1);
    };

    // Get the location to teleport to
    let location = match &args.location {
        Some(name) => {
            let matches = search_locations(name);
            handle_search_results(name, matches)
        }
        None => {
            // Default to first location (Hunter's Dream)
            &get_all_locations()[0]
        }
    };

    println!("Teleporting to: {} in {}", location.name, location.region);

    // Perform the teleport
    match teleport(&save_file, location) {
        Ok(()) => {
            println!("\nSuccessfully teleported to {}!", location.name);
            println!("Save file updated: {:?}", &save_file);
        }
        Err(e) => {
            println!("Error: Failed to teleport: {:?}", e);
            std::process::exit(1);
        }
    }
}
