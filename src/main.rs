use std::env;
use getopt::Opt;

fn print_usage() {
    println!("Usage: retouch [options] <include_files> [- <exclude_files>]");
    println!("\tOptions:");
    println!("\t-c\tSet creation date");
    println!("\t-m\tSet modification date");
    println!("\t-a\tSet last access date");
    println!("\t-h\tPrint this help");
    println!("");
    println!("\tThe c, m, and a could be combined, for example: retouch -rc *.jpg");
    println!("");
    println!("\t<include_files> - one or more file specification (name or wildcard) to change date/time");
    println!("\t<exclude_files> - One or more file specification (filename or wildcard) to skip from <include_files> list");
}

#[derive(Debug)]
struct Arguments {
    c_ : bool,
    m_ : bool,
    a_ : bool,
    includes_ : Vec<String>,
    excludes_ : Vec<String>
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments { c_: false, m_: false, a_: false, includes_: vec![], excludes_: vec![] }
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
        if !(self.a_ | self.c_ | self.m_) {
            self.a_ = true;
            self.c_ = true;
            self.m_ = true;
        }
    }
    fn parse() -> Result<Arguments, Error> {
        let mut args : Vec<String> = env::args().collect();
        let mut opts = getopt::Parser::new(&args[..], "cmah");

        let mut value : Arguments = Arguments { ..Default::default()};

        loop {
            match opts.next().transpose()? {
                None => break,
                Some(opt) => match opt {
                    Opt('a', None) => value.a_ = true,
                    Opt('c', None) => value.c_ = true,
                    Opt('m', None) => value.m_ = true,
                    Opt('h', None) => return Err(Error{}),
                    _ => unreachable!(),
                },
            }
        }

        value.set_flags_if_unset();



        let mut args = args.split_off(opts.index());

        if let Some(index) = args.iter().position(|x| x == "-") {
            let mut excl_args = args.split_off(index);
            let mut excl_args = excl_args.split_off(1);
            value.excludes_.append(&mut excl_args);
            
        }

        value.includes_.append(&mut args);

        return Ok(value);
    }
}
fn main() {

    let args = Arguments::parse();

    if args.is_err() {
        print_usage();
    }

    println!("{:?}", args);

}
