#![feature(box_syntax)]

extern crate hyper;
extern crate getopts;

//mod package;
//mod index;
//mod system;
//mod install;
//mod trust;

fn download_package(option: &Option<String>) 
{
    match option {
        &Some(ref x) => println!("Downloading {}", x),
        &None        => panic!("No input provided to S"),
    }
}

fn print_usage(context: &Context) 
{
    let brief = format!("Usage: {} PACKAGE [options]", context.get_prog_name());
    print!("{}", context.opts.usage(&brief));
}

struct Context
{
  args   : std::vec::Vec<std::string::String>,
  opts   : getopts::Options,
  matches: getopts::Matches
}

impl Context
{
  fn new(args: &std::vec::Vec<std::string::String>) -> Context
  {
    let mut opts = getopts::Options::new();
    opts.optopt( "S", "", "download package", "PACKAGE");
    opts.optflag("v", "version", "version 0.1");
    opts.optflag("h", "help", "print this help menu");

    let matches = opts.parse(&args[1..]).unwrap();

    Context {args: args.clone(), opts: opts, matches: matches}
  }

  fn get_prog_name(&self) -> &std::string::String
  {
    &self.args[0]
  }
}

fn main()
{

    let args: Vec<String> = std::env::args().collect();
    let context = Context::new(&args);

    if context.matches.opt_present("h") || !context.matches.free.is_empty() {
        print_usage(&context);
    } 
    else if context.matches.opt_present("v") {
      println!("Version: {}", 0.1);
    }
    else if context.matches.opt_present("S") {
      download_package(&context.matches.opt_str("S"));
    }
    std::process::exit(0);
}
