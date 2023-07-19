pub enum Menu {
    Main,
    Settings,
    Help,
}

pub enum Select {
    None,
    Main(MainMenu),
    Settings(SettingMenu),
    Help(HelpMenu),
}

/// 主菜单
pub enum MainMenu {
    Account,
    Preferences,
    Quit,
}

/// 设置子菜单  
pub enum SettingMenu {
    Theme,
    Language,
    Back,
}

/// 帮助子菜单
pub enum HelpMenu {
    About,
    Contact,
    Back,
}

