///! # sysfetch-cli
///!
///! Standalone Command LIne (CLI) frontend for fetching a userland of an OS into a desired directory using sysfetch crate's functions

// FIXME-QA(Krey): Consider implementing SNAFU <https://github.com/shepmaster/snafu> for better errors
// FIXME-BRAINSTORM(Krey): Consult with the torproject how to write software that is friendly to onion-services

/// Function to get userland of Exherbo Linux in specified directory
// TODO(Krey): See chapter in rustbook on slices as `str` is a slice thus we need `&str`
// DNC(Krey): Working on this, contrib of shims appreciated
sysfetch_exherbo(destDir: &str) {
	unimplemented!("Not implemented");

	// Download the tarball
	fetch!("https://dev.exherbo.org/stages/exherbo-x86_64-pc-linux-gnu-current.tar.xz")
		// DNM(Krey): Do not hard-code get the checksum from <https://dev.exherbo.org/stages>
		.sha256sum("f6bb1b9bf7067b193494de9b8a0f9c845f45461425646aa178bd3b2b2251836e")
		// FIXME-QA(Krey): Use destDir here
		.destination(cache_dir + project_name + file_name)

	// Extract the tarball
	extract!(cache_dir + project_name + "exherbo-x86_64-pc-linux-gnu-current.tar.xz")
		// DNM(Krey): Determine the default path for extracting the tarball
		.destination(matches.value_of("TARGET").unwrap_or("TBD"))
}

extern crate clap;
use clap::Clap;

extern crate directories_next;
use directories_next::BaseDirs;

fn main() {
	let matches = App::new("sysfetch-cli")
		.version("0.0.0")
		.author("Jacob Hrbek <kreyren+sysfetch@fsfe.org>")
		// DNM-DOCS(Krey): Requires a description
		.about("ABOUT-MESSAGE")
		.arg(
			Arg::new("no-cache")
				// FIXME(Krey): Is -N a good option for --no-cache?
				.short('N')
				.long("no-cache")
				// FIXME-DOCS(Krey): Not descriptive enough
				.about("Sets a custom config file"))
		// DNM-QA(Krey): Check if this works for the option
		#[cfg(feature = "torrent")]
		.arg(
			Arg::new("torrent")
				// FIXME(Krey): Is -t a good option here?
				.short('t')
				.long("torrent")
				// FIXME-DOCS(Krey): Not descriptive enough
				.about("Use torrent to fetch the system"))
		.arg(
			// FIXME-QA(Krey): Output a list of available variants in help message
			Arg::new("VARIANT")
				// FIXME-DOCS(Krey): Not descriptive enough
				.about("Get specified variant")
				.required(true)
				.takes_value(true)
				.index(1))
		.arg(
			Arg::new("TARGET")
				// FIXME-DOCS(Krey): Not descriptive enough
				.about("Sets a target directory to which we fetch the VARIANT")
				.required(false)
				.index(2))
		.arg(
			Arg::new("v")
				.short('v')
				.multiple(true)
				.about("Sets the level of verbosity"))
		.get_matches();


	// FIXME-QA(Krey): Ugly.. Can't we loop all arguments and if any of them is set just set 'ARGUMENT_<argument>' ?
	// FIXME-IMPL_DEP(Krey): This implementation is depending on clap.rs
	if matches.is_present("no-cache") {
		// FIXME-QA(Krey): Requires convention 'no-cache' -> 'no_cache', which is unwanted
		let ARGUMENT_no_cache = true;
	}

	if matches.is_present("torrent") {
		let ARGUMENT_torrent = true;
	}

	if matches.is_present("v") {
		// FIXME-IMPL(Krey): Implement
		unimplemented!("Verbosity is not implemented");
		let ARGUMENT_torrent = true;
	}

	match let variant = matches.value_of("VARIANT").to_lowercase() {
		exherbo => sysfetch_exherbo(),
		debian => sysfetch_debian(),
		_ => {
			// DNC-CONTRIB(Krey): Implement handling of strings to die crate to handle cross-platform
			die(synerr; "Variant '{:1}' is not recognized", variant)
		}
	}
}