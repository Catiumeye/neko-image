use clap::{Arg, ArgGroup, Command};
use neko_image::{args::ARG_OUTPUT, handlers};

fn main() {
  	let rpc1 = "\x1b[0m\x1b[48;2;92;195;233m\n";
  	let rpc2 = "\x1b[0m\x1b[48;2;235;165;177m\n";
  	let rpc3 = "\x1b[0m\x1b[48;2;255;255;255m\n";
  	let rust_programmer_colors = format!("{}{}{}{}{}\x1b[0m", rpc1, rpc2, rpc3, rpc2, rpc1);
  
	let help_template = color_print::cstr!("\
<bold>{name}</bold> v{version}\0
<bold>Author:</bold> <bold>{author}</bold>\
{about}\n\
{usage}\n\
{all-args}{after-help}
	");

  	let mathches = Command::new("NekoImage")
    	.after_help(rust_programmer_colors)
    	.version("0.0.3")
    	.subcommand_required(true)
    	.arg_required_else_help(true)
    	.author(color_print::cstr!("Catiumeye"))
		.help_template(help_template)
    	.subcommand(
      	Command::new("compress")
        	.about("Compress the image as you wish")
        	.arg(
          		Arg::new("relative_path")
            	.short('p')
            	.long("path")
            	.help("Relative path to image")
        	)
        	.arg(
          		Arg::new("absolute_path")
            	.short('a')
            	.long("apath")
            	.help("Absolute path to image")
        	)
        	.group(
          		ArgGroup::new("path_group")
            	.args(["relative_path", "absolute_path"])
            	.required(true)
        	)
        	.arg(
          		Arg::new("exact")
            	.short('e')
            	.long("exact")
            	.required(false)
            	.help("Range value 0...100 for compression index")
        	)
        	.arg(
				ARG_OUTPUT.to_owned()
        	),
    	)
    	.subcommand(
    	Command::new("ascii")
			.about("Make ASCII image from original picture")
			.arg(
				Arg::new("path")
				.short('p')
				.long("path")
				.required(true)
				.help("Path to image")
				.value_parser(clap::value_parser!(std::path::PathBuf))
			)
			.arg(
				Arg::new("gamma")
				.short('g')
				.long("gamma")
				.help("Image gamma 0..666 floating number")
				.default_value("0.6")
			)
			.arg(
				Arg::new("height")
				.short('y')
				.long("height")
				.required(false)
				.help("Height of output ascii image")
				.value_parser(clap::value_parser!(u32))
				.default_value("80")
			)
			.arg(
				Arg::new("width")
				.short('x')
				.long("width")
				.required(false)
				.help("Width of output ascii image")
				.value_parser(clap::value_parser!(u32))
				.default_value("100")
			)
    	)
    .get_matches();

  	let start_time = std::time::Instant::now();

  	match mathches.subcommand() {
    	Some(("compress", found_match)) => {
      		handlers::compress(found_match);

      		println!("Compressed in {:?}", std::time::Instant::now().duration_since(start_time));
    	},
		Some(("ascii", found_match)) => {
			handlers::make_ascii(found_match);

			println!("The result was written to the clipboard📋");
			println!("ASCII image created in {:?}", std::time::Instant::now().duration_since(start_time));
		}
    	a => {
      		panic!("Nothing: {:?}", a)
    	}
  	}
}