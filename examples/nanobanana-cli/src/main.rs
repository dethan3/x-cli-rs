use std::{path::PathBuf, time::Duration};

use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;
use xcli_browser::Browser;
use xcli_nanobanana::{gen as nanobanana_gen, GenOptions, GenOutput};
use xcli_output::{print_json, JsonResponse};
use xcli_webbridge::WebBridgeClient;

const DEFAULT_BRIDGE_URL: &str = "http://127.0.0.1:10086";
const SESSION_NAME: &str = "nanobanana-cli";

#[derive(Debug, Parser)]
#[command(name = "nanobanana-cli")]
#[command(about = "Generate images through Gemini Nano Banana using kimi-webbridge")]
struct Cli {
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Gen(GenArgs),
    #[command(alias = "generate")]
    Generate(GenArgs),
}

#[derive(Debug, Parser)]
struct GenArgs {
    prompt: String,

    #[arg(short, long, default_value = ".")]
    out: PathBuf,

    #[arg(long, default_value_t = 256)]
    thumb_width: u32,

    #[arg(long, default_value_t = 300)]
    timeout: u64,

    #[arg(long, env = "XCLI_WEBBRIDGE_URL", default_value = DEFAULT_BRIDGE_URL)]
    bridge_url: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    init_tracing(cli.verbose);

    let result = match cli.command {
        Commands::Gen(args) | Commands::Generate(args) => run_gen(args).await,
    };

    match result {
        Ok(data) => {
            let _ = print_json(&JsonResponse::ok(data));
        }
        Err(err) => {
            let _ = print_json(&JsonResponse::<()>::error(err.code(), err.to_string()));
            std::process::exit(1);
        }
    }
}

fn init_tracing(verbose: bool) {
    if !verbose {
        return;
    }

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(std::io::stderr)
        .without_time()
        .try_init();
}

async fn run_gen(args: GenArgs) -> xcli_core::Result<GenOutput> {
    let bridge = WebBridgeClient::with_session(args.bridge_url, SESSION_NAME);
    let browser = Browser::new(bridge);

    nanobanana_gen(
        &browser,
        GenOptions {
            prompt: args.prompt,
            out_dir: args.out,
            thumb_width: args.thumb_width,
            timeout: Duration::from_secs(args.timeout),
        },
    )
    .await
}
