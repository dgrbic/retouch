use std::{path::{PathBuf, Path}, collections::HashMap};
use globwalk;
use enumset::{self, EnumSetType, EnumSet};
use quickexif;
use chrono::{DateTime, TimeZone, Local};
use std::fs;
use anyhow::{Result, anyhow};
use filetime_creation::{FileTime, set_file_atime, set_file_mtime, set_file_ctime};
use colored::{Colorize, Color};
use std::collections::HashSet;
use clap::Parser;

// pub fn print_usage() {
//     println!("Usage: retouch [options] <include_files> [- <exclude_files>]");
//     println!("\tOptions:");
//     if cfg!(windows) { println!("\t-c\tSet creation date"); }
//     println!("\t-m\tSet modification date");
//     println!("\t-a\tSet last access date");
//     println!("\t-l\tLists files, displaying EXIF embedded creation date. Other date flags are ignored, no changes are applied.");
//     println!("\t-h\tPrint this help");
//     println!("");
//     println!("\tThe {}m and a could be combined, for example: retouch -rc *.jpg",  if cfg!(windows) {"c, "} else {""}  );
//     println!("");
//     println!("\t<include_files> - one or more file specification (name or wildcard) to change date/time. Defaults to '*'");
//     println!("\t<exclude_files> - One or more file specification (filename or wildcard) to skip from <include_files> list");
// }

#[derive(EnumSetType, Debug)]
#[cfg(windows)]
pub enum OptEnum {
    A, M, C
}
#[derive(EnumSetType, Debug)]
#[cfg(not(windows))]
pub enum OptEnum {
    A, M
}

#[derive(Debug, PartialEq, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Set creation date.
    #[cfg(windows)]
    #[arg(short)]
    c : bool,

    /// Set modification date.
    #[arg(short)]
    m : bool,

    /// Set last access date.
    #[arg(short)]
    a : bool,

    /// Lists files, displaying EXIF embedded creation date. Other date flags are ignored, no changes are applied.
    #[arg(short, long)]
    list : bool,

    /// One or more file specification (name or wildcard) to change date/time. Defaults to '*'.
    files : Vec<String>,

    /// One or more file specification (filename or wildcard) to skip from <include_files> list.
    #[arg(last = true)]
    exclude_files: Vec<String>,

    ///  Additional note
}

impl Args {
    #[cfg(windows)]
    fn is_ok(&self) -> bool {
        self.m || self.a || self.list ||  self.c 
    }

    #[cfg(not(windows))]
    fn is_ok(&self) -> bool {
        self.m || self.a || self.list 
    }

    fn set_flags_if_unset(&mut self) {
        if self.flags().is_empty() {
            self.m = true;
            self.a = true;
            if cfg!(windows) {
                self.c = true;
            }
        }
    }
    fn unset_flags(&mut self) {
        self.m = false;
        self.a = false;
        if cfg!(windows) {
            self.c = false;
        }
    }

    fn parse_vec(v : Vec<String>) -> Self {
        let mut args = Args::parse_from(v);
        if args.files.is_empty() {
            args.files.push("*".to_string());
        }
        if args.list {
            args.unset_flags();
        }
        else {
            args.set_flags_if_unset();
        }

        return args;
    }

    fn flags(&self) -> EnumSet<OptEnum> {
        let mut flags = EnumSet::new();
        if self.m {
            flags |= OptEnum::M;
        }
        if self.a {
            flags |= OptEnum::A;
        }
        if cfg!(windows) && self.c {
            flags |= OptEnum::C;
        }

        return flags;
    }

}

#[derive(Debug)]
pub struct App {
    args: Args,
    files : Vec<PathBuf>
}

impl Default for App {
    fn default() -> Self {
        App {
            args : Args { m: false, a: false, c: false, list: false, files: Vec::new(), exclude_files: Vec::new() },
            files : vec![]
        }
    }
}

impl App {
    pub fn get_options(&self) -> EnumSet<OptEnum> {
        self.args.flags()
    }
    pub fn create(args: Vec<String>) -> Result<App> {
        let mut app = App{..Default::default()};

        app.args = Args::parse_vec(args);

        if !app.args.is_ok() {
            return Err(anyhow!("help"));
        }

        let mut fileset = HashSet::new();

        for glob in &app.args.files {

            let path = Path::new(glob);
            if path.try_exists().unwrap_or(false) { 
                if path.is_file() {
                    fileset.insert(path.to_path_buf());
                }
                else if path.is_dir() {
                    match fs::read_dir(path) {
                        Ok(entries) => {
                            for entry in entries {
                                if entry.is_ok() {
                                    fileset.insert(entry.unwrap().path());
                                }
                            }
                        },
                        Err(_) => { },
                    }
                }
            }
            else {
                let base = path.parent().and_then(|p| p.canonicalize().ok()).unwrap_or(PathBuf::from(".")); // Should not return default as root folder is_dir and exists
                let this_base = Path::new(".").canonicalize().unwrap();
                let base = match base.as_path().strip_prefix(this_base.as_path()) {
                    Ok(b) => b.to_path_buf(),
                    Err(_) => base
                };
                let name = path.file_name().map_or("*", |f| f.to_str().unwrap() );
                let walker = globwalk::GlobWalkerBuilder::from_patterns(
                        base,
                        &[name]
                    )
                    .follow_links(true)
                    .case_insensitive(if cfg!(windows) { true } else { false } )
                    .build()?
                    .into_iter()
                    .filter_map(Result::ok);

                let files : Vec<PathBuf> = walker.filter(|f| f.file_type().is_file()).map(|d| d.into_path()).collect();
                for p in &files {
                    fileset.insert(p.to_path_buf());
                }
            }
        }
        for glob in &app.args.exclude_files {

            let path = Path::new(glob);
            if path.try_exists().unwrap_or(false) {
                if path.is_file() {
                    fileset.remove(&path.to_path_buf());
                }
                else if path.is_dir() {
                    match fs::read_dir(path) {
                        Ok(entries) => {
                            for entry in entries {
                                if entry.is_ok() {
                                    if let Ok(p) = entry {
                                        fileset.remove(&p.path());
                                    }
                                }
                            }
                        },
                        Err(_) => {},
                    }
                }
            }
            else {
                let base = path.parent().and_then(|p| p.canonicalize().ok()).unwrap_or(PathBuf::from(".")); // Should not return default as root folder is_dir and exists
                let this_base = Path::new(".").canonicalize().unwrap();
                let base = match base.as_path().strip_prefix(this_base.as_path()) {
                    Ok(b) => b.to_path_buf(),
                    Err(_) => base
                };
                let name = path.file_name().map_or("*", |f| f.to_str().unwrap() );
                let walker = globwalk::GlobWalkerBuilder::from_patterns(
                        base,
                        &[name]
                    )
                    .follow_links(true)
                    .case_insensitive(if cfg!(windows) { true } else { false } )
                    .build()?
                    .into_iter()
                    .filter_map(Result::ok);

                let files : Vec<PathBuf>  = walker.filter(|f| f.file_type().is_file()).map(|d| d.into_path()).collect();
                for p in &files {
                    fileset.remove(p);
                }
            }
        }


        app.files = fileset.into_iter().collect();
        Ok(app)
    }

    fn get_file_date(file : &PathBuf) -> Result<DateTime<Local>> {
        let file = fs::read(file.as_path())?;

        let rule = quickexif::describe_rule!(tiff {
                0x8769 {
                    0x9004 { str + 0 / create_date }
                }
            });

        let parsed_info = quickexif::parse(&file, &rule)?;
        let create_date = parsed_info.str("create_date")?;

        if let Ok(datetime) = create_date.parse::<DateTime::<Local>>() {
            return Ok(datetime);
        }
        else if let Ok(datetime) = Local.datetime_from_str(create_date, "%Y:%m:%d %H:%M:%S") {
            return Ok(datetime);
        }

        Err(anyhow!("EXIF create_date format {} is unknown.", create_date))
    }

    pub fn list_files(&self) {

        for file in &self.files[..] {
            if let Some(datetime) = App::get_file_date(&file).ok() {
                println!("{} : {}", datetime.to_string(), file.display());
            }
        }

    }

    pub fn apply_touch(&self) {
        for file in &self.files[..] {
            if let Some(datetime) = App::get_file_date(&file).ok() {
                let touch_date = FileTime::from_unix_time(datetime.timestamp(),  datetime.timestamp_subsec_nanos());
                let mut results = HashMap::new();
                for arg in self.args.flags() {
                    let tmp = match arg {
                        OptEnum::A => ('A' , set_file_atime(file, touch_date) ),
                        OptEnum::M => ('M' , set_file_mtime(file, touch_date) ),
                        OptEnum::C => ('C' , set_file_ctime(file, touch_date) ),
                    };

                    results.insert(tmp.0, tmp.1);
                }

                for r in results {
                    if !cfg!(windows) && r.0 == 'C' {
                        continue;
                    }
                    print!("{}", format!("{}", r.0).color(if r.1.is_ok() { Color::Green } else { Color::Red }));
                }

                println!(" : {}", file.display());
            }
        }        
    }
}

#[cfg(test)]
mod app_test;