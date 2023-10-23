use colored::Colorize;
use std::fs::{File, OpenOptions};
use std::path::{PathBuf, Path};
use std::time::Duration;

use fastq2comp::extract_comp::{run, FASTQReader, SampleArgs};
use fastq2comp::{io_utils, BaseComp};
use log::{error, info, warn, debug};
use server::{Plot, get_script_path, serialize_comps_for_script, run_script};
use simple_logger::SimpleLogger;

use std::io::{BufReader, Write};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Librarian CLI",
    about = "A tool to predict the sequencing library type from the base composition of a supplied FastQ file. Uncompresses .gz files when reading.",
)]
struct Cli {
    /// List of input files
    #[structopt(required = true, parse(from_os_str))]
    pub input: Vec<PathBuf>,

    /// Output directory (eg. `output_dir/`)
    /// 
    #[structopt(short = "o", long, parse(from_os_str), default_value = "")]
    pub output_dir: PathBuf,

    /// Suppresses all output except errors
    #[structopt(short, long)]
    pub quiet: bool,

    /// Specifies query URL to send prediction request to.
    /// Defaults to Babraham Bioinformatic's server.
    /// Passed argument is given precedence over environment variable.
    /// 
    /// If --local is set, this argument is ignored.
    /// 
    #[structopt(long, env = "LIBRARIAN_API_URL", default_value = "https://www.bioinformatics.babraham.ac.uk/librarian/api/plot_comp")]
    pub api: String,

    /// Run all processing locally, replacing the need for a server.
    /// Requires Rscript and other dependencies to be installed, along with the `scripts` folder.
    /// See https://github.com/DesmondWillowbrook/Librarian/blob/master/cli/README.md for more details. 
    /// 
    /// This cannot be set together with `api`.
    /// 
    #[structopt(short, long, conflicts_with("api"))]
    pub local: bool,
}

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

fn main() {
    let args = Cli::from_args();

    // p -> q equivalent to (!p or q)
    assert!((!args.local || args.api.is_empty()), "If running locally, API URL must not be set.");

    SimpleLogger::new()
        .with_level (if args.quiet {log::LevelFilter::Error} else {log::LevelFilter::Info})
        .env()
        .with_colors(true)
        .without_timestamps()
        .init()
        .expect("Couldn't initialize logger");

    query(args);
}

fn query(args: Cli) {
    let mut comps = Vec::with_capacity(args.input.len());

    for p in args.input {
        info!("Processing {p:?}");

        let f = File::open(&p);
        if let Err(e) = f {
            error!("Couldn't open {:?} (canonicalized: {:?}) for reading due to error {}", p, p.canonicalize(), e);
            continue;
        }
        let f = f.unwrap();
        let reader =
            io_utils::compressed_reader(f, matches!(p.extension(), Some(ext) if ext == "gz"));

        let comp = run(FASTQReader::new(
            SampleArgs::default(),
            BufReader::new(reader),
        ));

        let l = comp.reads_read();
        let target_len = SampleArgs::default().target_read_count;

        if l < target_len {
            warn!("Fewer valid reads ({l}) in sample {p:?} than recommended (100,000) (this may be due to reads being filtered out due to being shorter than 50 bases)");

            if l == 0 {
                error!("No valid reads found, skipping sample.");
                continue
            }
        }

        comps.push(comp);
    }

    if comps.is_empty() {
        error!("No samples could be processed.");
        return
    }

    debug!("Compositions: {:#?}", comps);

    if args.local {
        let mut working_dir = PathBuf::from(&args.output_dir);
        if working_dir.is_relative() {
            working_dir = std::env::current_dir().unwrap().join(&args.output_dir);
        }

        query_local(comps, &working_dir);
        info!("{} {:?}", "Created files in ".green(), &working_dir);
    } else {
        let res = query_server(args.api, comps);

        for res in res.into_iter() {
            let r = res.plot;
    
            let p = {
                let mut p = PathBuf::from(&args.output_dir);
                p.push(res.filename);
                p
            };
            let mut f = OpenOptions::new()
                .create(true)
                .write(true)
                .open(&p)
                .unwrap();
            f.write_all(&r).unwrap();
            info!("{} {:?}", "Created".green(), p);
        }
    };
}

fn query_server(url: String, comps: Vec<BaseComp>) -> Vec<Plot> {
    info!("Sending data to server at {url}");
    info!(
        "{}",
        "Requests may take up to 5 minutes to process.".green()
    );

    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(60 * 5))
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    let req = client.post(&url).json(&comps).send();

    let res = req.map_err(|e| {error!("{}\n", "Request to server failed".to_string().red()); panic!("{}", e)}).unwrap();
    if !res.status().is_success() {
        error!(
            "non-success response {} received, terminating",
            res.status().to_string().red()
        );
        error!("error body: {}", res.text().unwrap());
        panic!();
    }

    res
        .json::<Vec<Plot>>()
        .expect("unable to extract JSON from server response. server may be down")
}

fn query_local(comps: Vec<BaseComp>, working_dir: &Path) {
    assert!(!comps.is_empty());

    let input = serialize_comps_for_script(comps);

    let scripts_path = get_script_path();

    run_script(&scripts_path, working_dir, input).expect("R script should be successful");
}