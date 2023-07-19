mod models;

use models::menu::{HelpMenu, MainMenu, Menu, Select, SettingMenu};
fn main() {
    let mut menu = Menu::Main;
    let mut selection = Select::None;

    loop {
        println!("当前菜单:{:?}", menu);

        // 打印当前菜单选项
        match menu {
            Menu::Main => {
                print_menu(&MainMenu::variants());
            }
            Menu::Settings => {
                print_menu(&SettingMenu::variants());
            }
            Menu::Help => {
                print_menu(&HelpMenu::variants());
            }
        }

        // 读取用户选择
        let input = read_input();

        // 处理选择,切换菜单或执行操作
        match menu {
            Menu::Main => {
                selection = Select::Main(match input {
                    1 => MainMenu::Account,
                    2 => MainMenu::Preferences,
                    3 => MainMenu::Quit,
                    _ => MainMenu::Quit,
                });
            }
            Menu::Settings => {
                selection = Select::Settings(match input {
                    1 => SettingMenu::Theme,
                    2 => SettingMenu::Language,
                    3 => SettingMenu::Back,
                    _ => SettingMenu::Back,
                });
            }
            Menu::Help => {
                selection = Select::Help(match input {
                    1 => HelpMenu::About,
                    2 => HelpMenu::Contact,
                    3 => HelpMenu::Back,
                    _ => HelpMenu::Back,
                });
            }
        }

        // 根据选择切换菜单或退出
        match selection {
            Select::Main(item) => match item {
                MainMenu::Quit => break,
                MainMenu::Preferences => menu = Menu::Settings,
                MainMenu::Account => println!("切换到 Account"),
                _ => {}
            },
            // 其它菜单的处理
            _ => {}
        }
    }
}

// 打印菜单
fn print_menu(items: &[MainMenu]) {
    for (i, item) in items.iter().enumerate() {
        println!("{}. {:?}", i + 1, item);
    }
}

// 读取输入
use std::io;

fn read_input() -> u32 {
    let mut input = String::new();

    print!("请输入选择:");

    io::stdin().read_line(&mut input).expect("读取输入错误");

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("无效输入,请重试!");
            read_input() // 递归调用直到输入正确
        }
    };

    input
}
