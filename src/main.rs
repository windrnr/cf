use std::{env, error::Error, process, io, iter::zip, fs};
use colored::Colorize;

pub struct FileInfo {
    pub name: String,
    pub content: String,
}

impl FileInfo {
    pub fn open(mut args: impl Iterator<Item = String>) -> Result<(FileInfo, FileInfo), Box<dyn Error>> {
        let file1 = match args.next() {
            Some(file) => file,
            None => return Err("Uso: cf <file1> <file2>".into()),
        };

        let file2 = match args.next() {
            Some(file) => file,
            None => return Err("Uso: cf <file1> <file2>".into()),
        };

        let content_file1 = fs::read_to_string(&file1)?;
        let content_file2 = fs::read_to_string(&file2)?;

        let f1 = FileInfo { name: file1, content: content_file1};
        let f2 = FileInfo { name: file2, content: content_file2};
        Ok((f1, f2))
    }
}

fn diff_info(file_name: &String,) -> String {
    format!("{} {}:line:column:\t", "-->".to_string().cyan().bold(), file_name)
}

fn compare(f1: FileInfo, f2: FileInfo) -> io::Result<()> {
    let content_file1 = f1.content.lines().collect::<Vec<&str>>();
    let content_file2 = f2.content.lines().collect::<Vec<&str>>();

    for (line1, line2) in zip(content_file1, content_file2) {
        if line1 != line2 {
            let line1 = format!("{}{}\n", diff_info(&f1.name), line1.to_string().green());
            let line2 = format!("{}{}\n", diff_info(&f2.name), line2.to_string().red());
            println!("{line1}{line2}")
        }
    }

    Ok(())
}

fn main() {
    let mut args = env::args();
    args.next();

    if args.len() > 2 {
        eprintln!("Uso: cf <file1> <file2>");
        process::exit(64);
    }

    let (file1, file2) = FileInfo::open(args).unwrap_or_else(|err| {
        eprintln!("Error: {err}");
        process::exit(64);
    });
    
    if let Err(e) = compare(file1, file2) {
        eprintln!("Error: {e}");
        process::exit(64);
    }
}
