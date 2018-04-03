#[macro_use]

extern crate clap;

use clap::{App, Arg, ArgMatches};


/// Main function for this program when run as script.
fn notescan_main(options: ArgMatches) {
    if let Some(filenames) = options.values_of("inputs") {
        for filename in filenames {
            println!("file: {}", filename);
        }
    }
}


/// Parse args and call notescan_main().
fn main() {
    let quite = Arg::with_name("quite")
        .help("reduce program output")
        .short("q")
        .long("quite");

    let inputs = Arg::with_name("inputs")
        .required(true)
        .help("files to convert")
        .multiple(true)
        .takes_value(true);

    let output = Arg::with_name("output")
        .required(true)
        .help("output PDF filename")
        .short("o")
        .long("output")
        .takes_value(true);

    let global_palette = Arg::with_name("global_palette")
        .help("use one global palette for all pages")
        .short("g")
        .long("global");

    let sort_numerically = Arg::with_name("sort_numerically")
        .help("keep filenames ordered as specified;\n\
               use if you *really* want IMG_10.png to precede IMG_2.png")
        .short("K");

    let options = App::new("noteshrink")
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Convert scans of handwritten notes to beautiful, compact PDFs")
        .args(&[
            quite,
            global_palette,
            sort_numerically,
            output,
            inputs,
        ]);

    notescan_main(options.get_matches());
}
