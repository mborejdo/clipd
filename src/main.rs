
use clap::{arg, App, AppSettings};
use clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_master::{Master, ClipboardHandler, CallbackResult};
use std::{fs, io};

struct Handler;

fn write_message(path: &str, data: String) -> io::Result<()> {
    let max = 65;
    let content = data
        .replace(&[' ', '/', '{', '}', '?', ',', '\\', '\"', '.', ';', ':', '\''][..], "_")
        .replace(|c: char| !c.is_ascii(), "_")
        .replace('\n', "")
        .replace('\r', "");

    let filename = if content.len() > max { &content[0..max] } else { &content };
    let fh = fs::write(format!("{}{}{}", path,  "_", filename), content.clone());
    match fh {
        Ok(file) => file,
        Err(_error) => eprintln!("Problem opening the file: {:?}", filename),
    };
  

    Ok(())
}

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let clipboard_contents = ctx.get_contents();
        if clipboard_contents.is_ok() {
            let path: &str = "C:\\Users\\mib\\.dotfiles\\clip\\clips\\";
            let written = write_message(path, clipboard_contents.unwrap());
            match written {
                Ok(()) => println!("Clipboard change happened!"),
                Err(error) => eprintln!("Problem writing to {:?}", error),
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

    let matches = App::new("clipd")
        .about("clipboard awesomeness")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::AllowExternalSubcommands)
        .subcommand(
            App::new("run")
                .about("run things")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(arg!(<PATH> ... "Stuff to run").allow_invalid_utf8(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("run", sub_matches)) => {
            let paths = sub_matches
                .values_of_os("PATH")
                .unwrap_or_default()
                .collect::<Vec<_>>();

            let runner = Master::new(Handler).run();
            match runner {
                Ok(()) => println!("Runner OK"),
                Err(error) => println!("Problem running handler {:?}", error),
            }
        }
        _ => unreachable!(),
    }
    
}