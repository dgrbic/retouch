use std::{path::PathBuf, collections::HashMap};
use globwalk;
use getopt::Opt;
use enumset::{self, EnumSetType, EnumSet};
use quickexif;
use chrono::{DateTime, TimeZone, Local};
use std::fs;
use anyhow::{Result, anyhow};
use filetime_creation::{FileTime, set_file_atime, set_file_mtime, set_file_ctime};
use colored::{Colorize, Color};

pub fn print_usage() {
    println!("Usage: retouch [options] <include_files> [- <exclude_files>]");
    println!("\tOptions:");
    if cfg!(windows) { println!("\t-c\tSet creation date"); }
    println!("\t-m\tSet modification date");
    println!("\t-a\tSet last access date");
    println!("\t-l\tLists files, displaying EXIF embedded creation date. Other date flags are ignored, no changes are applied.");
    println!("\t-h\tPrint this help");
    println!("");
    println!("\tThe {}m and a could be combined, for example: retouch -rc *.jpg",  if cfg!(windows) {"c, "} else {""}  );
    println!("");
    println!("\t<include_files> - one or more file specification (name or wildcard) to change date/time. Defaults to '*'");
    println!("\t<exclude_files> - One or more file specification (filename or wildcard) to skip from <include_files> list");
}

#[derive(EnumSetType, Debug)]
#[cfg(windows)]
pub enum OptEnum {
    A, M, C
}
#[derive(EnumSetType, Debug)]
#[cfg(not(windows))]
pub enum OptEnum {
    A, M, C
}

#[derive(Debug, PartialEq)]
struct Arguments {
    opts_ : EnumSet<OptEnum>,
    globs_ : Vec<String>,
    show_help_ : bool
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments { opts_ : EnumSet::new(),  globs_: vec![] , show_help_: false }
    }
}

impl Arguments {
    fn create_help() -> Self {
        Arguments { opts_ : EnumSet::new(),  globs_: vec![] , show_help_: true }
    }
    fn set_flags_if_unset(&mut self) {
        if self.opts_.is_empty() {
            self.opts_ = OptEnum::A | OptEnum::C | OptEnum::M;
        }
    }
    fn unset_flags(&mut self) {
        self.opts_.clear();
    }
    fn flags(&self) -> EnumSet<OptEnum> {
        return self.opts_;
    }
    fn glob(&self) -> String {
        return self.globs_[..].join(",");
    }

    fn get_show_help(&self) -> bool {
        return self.show_help_ == true;
    }

    fn parse(args: Vec<String>) -> Result<Arguments> {
        let mut args = args;
        let mut opts = getopt::Parser::new(&args[..], "cmahl");

        let mut value : Arguments = Arguments { ..Default::default()};

        let mut list_flag = false;

        loop {
            match opts.next().transpose()? {
                None => break,
                Some(opt) => match opt {
                    Opt('a', None) => value.opts_ |= OptEnum::A,
                    Opt('c', None) => value.opts_ |= OptEnum::C,
                    Opt('m', None) => value.opts_ |= OptEnum::M,
                    Opt('l', None) => list_flag = true,
                    Opt('h', None) => value.show_help_ = true,
                    _ => unreachable!(),
                },
            }
        }

        if list_flag {
            value.unset_flags();
        }
        else {
            value.set_flags_if_unset();
        }

        let mut args = args.split_off(opts.index());

        if let Some(index) = args.iter().position(|x| x == "-") {
            let mut excl_args = args.split_off(index);
            value.globs_.append(&mut args);
            let mut excl_args = excl_args.split_off(1);
            let mut excl_args = excl_args.iter_mut().map(|s| "!".to_owned()+s).collect();
            value.globs_.append(&mut excl_args);
            
        }
       else {
            value.globs_.append(&mut args);
        }

        if value.glob().is_empty() {
            value.globs_.append(&mut vec!["*".to_string()]);
        }
        return Ok(value);
    }
}

#[derive(Debug)]
pub struct App {

    args: Arguments,
    files : Vec<PathBuf>
}

impl Default for App {
    fn default() -> Self {
        App {
            args : Arguments { ..Default::default() },
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
        let args = Arguments::parse(args);

        app.args = args.unwrap_or(Arguments::create_help());

        if app.args.get_show_help() {
            return Err(anyhow!("help"));
        }

        for glob in &app.args.globs_[..] {
            let walker = globwalk::glob(glob)?.into_iter().filter_map(Result::ok);
            let mut files = walker.filter(|f| f.file_type().is_file()).map(|d| d.into_path()).collect();
            app.files.append(&mut files);
        }

        Ok(app)
    }

    fn get_file_date(file : &PathBuf) -> Result<DateTime<Local>> {
        let file = fs::read(file.as_path())?;

            // the JPEG header will automatically be removed
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