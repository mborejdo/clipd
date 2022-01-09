use clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_master::{Master, ClipboardHandler, CallbackResult};
use std::{fs, io};

struct Handler;

impl ClipboardHandler for Handler {

    fn on_clipboard_change(&mut self) -> CallbackResult {
        println!("Clipboard change happened!");
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let r = ctx.get_contents();
        let path = "C:\\Users\\mib\\.dotfiles\\clip\\clips\\";
        let max = 65;

        if r.is_ok() {
            let content = r.unwrap()
                    .replace(&[' ',  '/', '\\', '\"', '.', ';', ':', '\''][..], "_")
                    .replace(|c: char| !c.is_ascii(), "_")
                    .replace('\n', "")
                    .replace('\r', "");

            let filename = if content.len() > max { &content[0..max] } else { &content };

            let fsw = fs::write(format!("{}{}", path, filename), content.clone());
            match fsw {
                Ok(file) => file,
                Err(_error) => eprintln!("Problem opening the file: {:?}", filename),
            };
        }

        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }
}

fn main() {
    println!("{}", "Started clipd");
    let _ = Master::new(Handler).run();
}