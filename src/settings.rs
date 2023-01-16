use bevy::{prelude::*, window::PresentMode};


pub struct SettingsPlugin;

#[derive(serde::Deserialize, serde::Serialize, bevy::reflect::TypeUuid)]
#[uuid = "b1e39643-8bc5-4faa-bbd2-b34b7eeaa336"]
#[derive(Resource)]
pub struct Settings {
    pub fullscreen: bool,
    pub vsync: bool,
}

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(load_apply_settings)
        ;
    }
}

impl Settings {
    fn settings_file_path() -> String {
        String::from("settings.ron")
    }
    
    fn default_settings() -> Settings {
        Settings {
            fullscreen: true,
            vsync: true,
        }
    }
    
    pub fn write_to_disk(&self) {
        let pretty = ron::ser::PrettyConfig::new()
            .depth_limit(2)
            .separate_tuple_members(true)
            .enumerate_arrays(true);
        
        let serialized = ron::ser::to_string_pretty(&self, pretty).unwrap();
        
        std::fs::write(&Settings::settings_file_path(), serialized).unwrap();
    }
}

fn load_apply_settings(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
) {       
    let mut successfully_loaded = false;
    let settings = if let Ok(text) = std::fs::read_to_string(Settings::settings_file_path()) {
        if let Ok(loaded) = ron::from_str(text.as_str()) {
            successfully_loaded = true;
            loaded
        }
        else {
            Settings::default_settings()
        }
    }
    else {
        Settings::default_settings()
    };
    
    if !successfully_loaded {
        settings.write_to_disk();
    }   
     
    let window = windows.primary_mut();
    
    window.set_present_mode(if settings.vsync { PresentMode::AutoVsync } else { PresentMode::AutoNoVsync });
    window.set_mode(if settings.fullscreen { WindowMode::Fullscreen } else { WindowMode::Windowed });
    
    commands.insert_resource(settings);
}
