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

fn load_apply_settings(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
) {
    let file_path = "settings.ron";
    let default_settings = Settings {
        fullscreen: true,
        vsync: true,
    };
        
    let settings = if let Ok(text) = std::fs::read_to_string(file_path) {
        ron::from_str(text.as_str()).unwrap_or(default_settings)
    }
    else {
        let serialized = ron::to_string(&default_settings).unwrap();
        std::fs::write(file_path, serialized).unwrap();
        
        default_settings
    };
    
    let window = windows.primary_mut();
    
    window.set_present_mode(if settings.vsync { PresentMode::AutoVsync } else { PresentMode::AutoNoVsync });
    window.set_mode(if settings.fullscreen { WindowMode::Fullscreen } else { WindowMode::Windowed });
    
    commands.insert_resource(settings);
    
    
}
