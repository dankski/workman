use cursive::CursiveRunnable;
use cursive::views::Dialog;
use cursive::views::TextView;

pub struct App {
    siv: CursiveRunnable,
    cfg: Configuration,
}

#[derive(Debug, Clone)]
pub struct Configuration {
    title: Option<String>,
}

impl Configuration {

    pub fn new() -> Self {
        Self {
            title: None,
        }
    }

    pub fn title<'a>(&'a mut self, title: &str) -> &'a mut Configuration {
        self.title = Some(title.to_string());
        self
    }
}

impl App {

    pub fn from(cfg: &Configuration) -> Self {
        Self{siv: cursive::default(), cfg: cfg.clone()}   
    }

    pub fn init(&mut self) {
        self.siv.add_layer(Dialog::around(TextView::new("Hello Dialog!"))
        .title(self.cfg.title.as_ref().unwrap_or(&String::from("Default")))
        .button(String::from("Quit"), |s| s.quit()));
    }

    pub fn run(&mut self) {
        self.siv.run();
    }

}


