use windows_registry::*;

pub fn is_light_theme() -> bool {
    let val = CURRENT_USER
        .open(r#"Software\Microsoft\Windows\CurrentVersion\Themes\Personalize"#)
        .expect("Failed to get registry key")
        .get_u32("AppsUseLightTheme")
        .expect("Failed to get registry value");
    val != 0
}

// pub fn is_dark_theme() -> bool {
//     !is_light_theme()
// }
