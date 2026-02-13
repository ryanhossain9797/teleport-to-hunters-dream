//! Application state management for the TUI

use std::path::PathBuf;

use lantern_teleport_core::{
    CurrentPosition, Location, get_all_locations, teleport, validate_save_file,
};

/// Application mode/state
#[derive(Debug, Clone)]
pub enum AppMode {
    /// File browser for selecting save file
    FileBrowser,
    /// Validating the selected save file
    Validating,
    /// Save file validation succeeded
    ValidationSuccess(CurrentPosition),
    /// Save file validation failed
    ValidationError(String),
    /// Selecting a teleport destination
    LocationSelection,
    /// Search mode for filtering locations
    LocationSearch,
    /// Confirming teleport action
    Confirmation(usize),
    /// Performing teleport
    Teleporting,
    /// Teleport succeeded
    TeleportSuccess,
    /// Teleport failed
    TeleportError(String),
}

/// Location grouped by region for display
#[derive(Debug, Clone)]
pub struct LocationGroup {
    pub region: &'static str,
    pub locations: Vec<&'static Location>,
}

/// Main application state
pub struct App {
    /// Current application mode
    pub mode: AppMode,
    /// Current directory path in file browser
    pub current_path: PathBuf,
    /// List of entries in current directory
    pub file_list: Vec<FileEntry>,
    /// Currently selected file index
    pub selected_file: usize,
    /// Scroll offset for file list
    pub file_scroll_offset: usize,
    /// Location groups (grouped by region)
    pub location_groups: Vec<LocationGroup>,
    /// Filtered location groups (when searching)
    pub filtered_location_groups: Vec<LocationGroup>,
    /// Currently selected location index (flat index across all groups)
    pub selected_location: usize,
    /// Scroll offset for location list
    pub location_scroll_offset: usize,
    /// Search query string
    pub search_query: String,
    /// Path to the validated save file
    pub save_file_path: Option<PathBuf>,
    /// Current position from save file
    pub current_position: Option<CurrentPosition>,
    /// Selected location for teleport
    pub selected_destination: Option<&'static Location>,
    /// Confirmation dialog selection (true = confirm, false = cancel)
    pub confirm_selection: bool,
    /// Should the app quit?
    pub should_quit: bool,
}

/// File entry in the file browser
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
}

impl App {
    /// Create a new application instance
    pub fn new() -> Self {
        let current_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        let location_groups = Self::group_locations_by_region();

        Self {
            mode: AppMode::FileBrowser,
            current_path: current_path.clone(),
            file_list: Vec::new(),
            selected_file: 0,
            file_scroll_offset: 0,
            location_groups: location_groups.clone(),
            filtered_location_groups: location_groups,
            selected_location: 0,
            location_scroll_offset: 0,
            search_query: String::new(),
            save_file_path: None,
            current_position: None,
            selected_destination: None,
            confirm_selection: false,
            should_quit: false,
        }
    }

    /// Group all locations by their region
    fn group_locations_by_region() -> Vec<LocationGroup> {
        let locations = get_all_locations();
        let mut groups: Vec<LocationGroup> = Vec::new();

        for location in locations {
            if let Some(existing) = groups.iter_mut().find(|g| g.region == location.region) {
                existing.locations.push(location);
            } else {
                groups.push(LocationGroup {
                    region: location.region,
                    locations: vec![location],
                });
            }
        }

        groups
    }

    /// Refresh the file list for the current directory
    pub fn refresh_file_list(&mut self) {
        self.file_list.clear();
        self.selected_file = 0;
        self.file_scroll_offset = 0;

        // Add parent directory entry
        if self.current_path.parent().is_some() {
            self.file_list.push(FileEntry {
                name: "..".to_string(),
                path: self.current_path.parent().unwrap().to_path_buf(),
                is_dir: true,
            });
        }

        // Read directory entries
        if let Ok(entries) = std::fs::read_dir(&self.current_path) {
            let mut dirs: Vec<FileEntry> = Vec::new();
            let mut files: Vec<FileEntry> = Vec::new();

            for entry in entries.flatten() {
                let path = entry.path();
                let name = path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();

                // Skip only . and .. (they're handled separately)
                if name == "." || name == ".." {
                    continue;
                }

                let is_dir = path.is_dir();
                let file_entry = FileEntry { name, path, is_dir };

                if is_dir {
                    dirs.push(file_entry);
                } else {
                    files.push(file_entry);
                }
            }

            // Sort directories and files alphabetically
            dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
            files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

            // Add directories first, then files
            self.file_list.extend(dirs);
            self.file_list.extend(files);
        }
    }

    /// Navigate to the selected file/directory
    pub fn navigate_to_selected(&mut self) {
        if let Some(entry) = self.file_list.get(self.selected_file) {
            if entry.is_dir {
                self.current_path = entry.path.clone();
                self.refresh_file_list();
            } else {
                // Validate the selected file
                self.save_file_path = Some(entry.path.clone());
                self.mode = AppMode::Validating;
            }
        }
    }

    /// Validate the selected save file
    pub fn validate_save_file(&mut self) {
        if let Some(ref path) = self.save_file_path {
            match validate_save_file(path) {
                Ok(position) => {
                    self.current_position = Some(position.clone());
                    self.mode = AppMode::ValidationSuccess(position);
                }
                Err(e) => {
                    self.mode = AppMode::ValidationError(format!("{:?}", e));
                }
            }
        }
    }

    /// Move file selection up
    pub fn move_file_up(&mut self) {
        if self.selected_file > 0 {
            self.selected_file -= 1;
        }
    }

    /// Move file selection down
    pub fn move_file_down(&mut self) {
        if self.selected_file < self.file_list.len().saturating_sub(1) {
            self.selected_file += 1;
        }
    }

    /// Move location selection up
    pub fn move_location_up(&mut self) {
        if self.selected_location > 0 {
            self.selected_location -= 1;
        }
    }

    /// Move location selection down
    pub fn move_location_down(&mut self) {
        let total = self.get_total_filtered_locations();
        if self.selected_location < total.saturating_sub(1) {
            self.selected_location += 1;
        }
    }

    /// Get total count of filtered locations
    pub fn get_total_filtered_locations(&self) -> usize {
        self.filtered_location_groups
            .iter()
            .map(|g| g.locations.len())
            .sum()
    }

    /// Get the currently selected location
    pub fn get_selected_location(&self) -> Option<&'static Location> {
        let mut current_idx = 0;
        for group in &self.filtered_location_groups {
            for location in &group.locations {
                if current_idx == self.selected_location {
                    return Some(*location);
                }
                current_idx += 1;
            }
        }
        None
    }

    /// Apply search filter to locations
    pub fn apply_search_filter(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_location_groups = self.location_groups.clone();
        } else {
            let query = self.search_query.to_lowercase();
            let mut filtered: Vec<LocationGroup> = Vec::new();

            for group in &self.location_groups {
                let matching: Vec<&Location> = group
                    .locations
                    .iter()
                    .filter(|loc| loc.name.to_lowercase().contains(&query))
                    .copied()
                    .collect();

                if !matching.is_empty() {
                    filtered.push(LocationGroup {
                        region: group.region,
                        locations: matching,
                    });
                }
            }

            self.filtered_location_groups = filtered;
        }

        // Reset selection if it's out of bounds
        let total = self.get_total_filtered_locations();
        if self.selected_location >= total && total > 0 {
            self.selected_location = total - 1;
        }
    }

    /// Clear the search filter
    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.apply_search_filter();
        self.mode = AppMode::LocationSelection;
    }

    /// Select the current location and show confirmation
    pub fn select_location(&mut self) {
        if let Some(location) = self.get_selected_location() {
            self.selected_destination = Some(location);
            self.confirm_selection = false;
            self.mode = AppMode::Confirmation(self.selected_location);
        }
    }

    /// Move confirmation selection left
    pub fn move_confirm_left(&mut self) {
        self.confirm_selection = false;
    }

    /// Move confirmation selection right
    pub fn move_confirm_right(&mut self) {
        self.confirm_selection = true;
    }

    /// Execute the teleport
    pub fn execute_teleport(&mut self) {
        if self.confirm_selection {
            if let (Some(path), Some(location)) = (&self.save_file_path, self.selected_destination)
            {
                self.mode = AppMode::Teleporting;
                match teleport(path, location) {
                    Ok(()) => {
                        self.mode = AppMode::TeleportSuccess;
                    }
                    Err(e) => {
                        self.mode = AppMode::TeleportError(format!("{:?}", e));
                    }
                }
            }
        } else {
            // Cancel - go back to location selection
            self.mode = AppMode::LocationSelection;
        }
    }

    /// Go back to file browser
    pub fn go_back_to_file_browser(&mut self) {
        self.mode = AppMode::FileBrowser;
        self.save_file_path = None;
        self.current_position = None;
        self.selected_destination = None;
        self.search_query.clear();
        self.selected_location = 0;
        self.apply_search_filter();
    }

    /// Quit the application
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
