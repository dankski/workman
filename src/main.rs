mod frontend;

fn main() {

    let mut cfg = frontend::Configuration::new();
    cfg.title("Working Man");

    let mut app = frontend::App::from(&cfg);
    app.init();
    app.run();
}
