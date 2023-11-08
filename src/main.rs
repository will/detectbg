use termbg::{Theme, theme};

fn main() {
    let timeout = std::time::Duration::from_millis(100);
    let theme = theme(timeout);

    let light_str = std::env::args().nth(1).unwrap_or("light".to_owned());
    let dark_str = std::env::args().nth(2).unwrap_or("dark".to_owned());

    match theme {
        Ok(Theme::Light) | Err(_) => {
            println!("{}", light_str);
        }
        Ok(Theme::Dark) => {
            println!("{}", dark_str);
        }
    }
}
