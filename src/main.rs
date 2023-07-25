use std::fs;
use std::env;
use std::process;

use eframe::egui;

#[derive(Default)]
struct Content{
    text: String,
}

impl eframe::App for Content{
    fn update(&mut self, context: &eframe::egui::Context, frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(context, |ui|{
            ui.heading("TESTING APP");
        });
    }
}


fn main()  -> eframe::Result<()>{

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "MultiGREP",
        options,
        Box::new(|_cc| Box::<Content>::default()),
    )

    // let args: Vec<String> = env::args().collect();
    // let directs = fs::read_dir("./").unwrap();
    //
    // let config = multigrep::Config::build(&args, directs).unwrap_or_else(|err| {
    //     eprintln!("Argument error: {err}");
    //     process::exit(1);
    // });
    //
    // if let Err(e) = multigrep::run(config){
    //     eprintln!("You messed up buddy (probably). {e}");
    //     process::exit(1);
    // }


}
