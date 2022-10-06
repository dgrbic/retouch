use std::io;    
use globwalk;
use walkdir::DirEntry;
use getopt::Opt;
use enumset::{self, EnumSetType, EnumSet};

pub fn print_usage() {
    println!("Usage: retouch [options] <include_files> [- <exclude_files>]");
    println!("\tOptions:");
    println!("\t-c\tSet creation date");
    println!("\t-m\tSet modification date");
    println!("\t-a\tSet last access date");
    println!("\t-l\tLists files, displaying EXIF embedded creation date. Other date flags are ignored, no changes are applied.");
    println!("\t-h\tPrint this help");
    println!("");
    println!("\tThe c, m, and a could be combined, for example: retouch -rc *.jpg");
    println!("");
    println!("\t<include_files> - one or more file specification (name or wildcard) to change date/time. Defaults to '*'");
    println!("\t<exclude_files> - One or more file specification (filename or wildcard) to skip from <include_files> list");
}

#[derive(EnumSetType, Debug)]
pub enum OptEnum {
    A, C, M
}

#[derive(Debug, PartialEq)]
struct Arguments {
    opts_ : EnumSet<OptEnum>,
    globs_ : Vec<String>,
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments { opts_ : EnumSet::new(),  globs_: vec![] }
    }
}

#[derive(Debug)]
struct Error {}

impl From<getopt::Error> for Error {
    fn from(_: getopt::Error) -> Error {
        Error{}
    }
}


impl Arguments {
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

    fn parse(args: Vec<String>) -> Result<Arguments, Error> {
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
                    Opt('h', None) => return Err(Error{}),
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

        if value.globs_.is_empty() {
            value.globs_.append(&mut vec!["*".to_string()]);
        }
        return Ok(value);
    }
}

#[derive(Debug)]
pub struct App {

    args: Arguments,
    files : Vec<DirEntry>
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
    pub fn create(args: Vec<String>) -> io::Result<App> {
        let mut app = App{..Default::default()};
        let args = Arguments::parse(args);

        if args.is_err() {
            //print_usage();
            return Err(io::Error::new(io::ErrorKind::Other, "Help"));
        }

        app.args = args.ok().unwrap();

        let walker = globwalk::GlobWalkerBuilder::from_patterns(".", &app.args.globs_[..]).max_depth(1).build()?.into_iter().filter_map(Result::ok);

        app.files = walker.filter(|f| f.file_type().is_file()).collect();

        Ok(app)

    }
}

#[cfg(test)]
mod app_test;