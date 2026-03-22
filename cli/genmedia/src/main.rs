mod client;
mod commands;
mod config;
mod request_params;
mod types;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ai-media")]
#[command(about = "AI generation CLI for agent workflows", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Config(ConfigCommand),
    Models(ModelsCommand),
    Image(ImageCommand),
    Video(VideoCommand),
    Task(TaskCommand),
}

#[derive(Args)]
struct ConfigCommand {
    #[command(subcommand)]
    command: ConfigSubcommand,
}

#[derive(Subcommand)]
enum ConfigSubcommand {
    SetKey { key: String },
    SetBaseUrl { base_url: String },
    Show,
}

#[derive(Args)]
struct ModelsCommand {
    #[command(subcommand)]
    command: ModelsSubcommand,
}

#[derive(Subcommand)]
enum ModelsSubcommand {
    List(ModelsListArgs),
    Show(ModelsShowArgs),
}

#[derive(Args)]
struct ModelsListArgs {
    #[arg(long)]
    json: bool,
}

#[derive(Args)]
struct ModelsShowArgs {
    #[arg(long)]
    model: String,
    #[arg(long, default_value_t = false)]
    json: bool,
}

#[derive(Args)]
struct ImageCommand {
    #[command(subcommand)]
    command: ImageSubcommand,
}

#[derive(Subcommand)]
enum ImageSubcommand {
    Generate(GenerateImageArgs),
    Get(GetTaskArgs),
}

#[derive(Args)]
struct VideoCommand {
    #[command(subcommand)]
    command: VideoSubcommand,
}

#[derive(Subcommand)]
enum VideoSubcommand {
    Generate(GenerateVideoArgs),
    Get(GetTaskArgs),
}

#[derive(Args)]
struct TaskCommand {
    #[command(subcommand)]
    command: TaskSubcommand,
}

#[derive(Subcommand)]
enum TaskSubcommand {
    Get(GenericTaskArgs),
}

#[derive(Args)]
struct GenerateImageArgs {
    #[arg(long)]
    model: String,
    #[arg(long)]
    prompt: String,
    #[arg(long)]
    aspect_ratio: Option<String>,
    #[arg(long)]
    response_format: Option<String>,
    #[arg(long = "image")]
    images: Vec<String>,
    #[arg(long)]
    metadata_json: Option<String>,
    #[arg(long = "param", value_name = "KEY=VALUE")]
    params: Vec<String>,
    #[arg(long, default_value_t = false)]
    wait: bool,
    #[arg(long, default_value_t = 5)]
    poll_interval: u64,
}

#[derive(Args)]
struct GenerateVideoArgs {
    #[arg(long)]
    model: String,
    #[arg(long)]
    prompt: String,
    #[arg(long)]
    aspect_ratio: Option<String>,
    #[arg(long)]
    duration: Option<u64>,
    #[arg(long = "image")]
    images: Vec<String>,
    #[arg(long = "param", value_name = "KEY=VALUE")]
    params: Vec<String>,
    #[arg(long, default_value_t = false)]
    wait: bool,
    #[arg(long, default_value_t = 8)]
    poll_interval: u64,
}

#[derive(Args)]
struct GetTaskArgs {
    #[arg(long)]
    task_id: String,
    #[arg(long, default_value_t = false)]
    wait: bool,
    #[arg(long, default_value_t = 5)]
    poll_interval: u64,
}

#[derive(Args)]
struct GenericTaskArgs {
    #[arg(long, value_parser = ["image", "video"])]
    kind: String,
    #[arg(long)]
    task_id: String,
    #[arg(long, default_value_t = false)]
    wait: bool,
    #[arg(long, default_value_t = 5)]
    poll_interval: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Config(command) => match command.command {
            ConfigSubcommand::SetKey { key } => commands::config::set_key(key).await,
            ConfigSubcommand::SetBaseUrl { base_url } => {
                commands::config::set_base_url(base_url).await
            }
            ConfigSubcommand::Show => commands::config::show().await,
        },
        Command::Models(command) => match command.command {
            ModelsSubcommand::List(args) => commands::models::list(args.json).await,
            ModelsSubcommand::Show(args) => commands::models::show(args.model, args.json).await,
        },
        Command::Image(command) => match command.command {
            ImageSubcommand::Generate(args) => {
                commands::image::generate(
                    args.model,
                    args.prompt,
                    args.aspect_ratio,
                    args.response_format,
                    args.images,
                    args.metadata_json,
                    args.params,
                    args.wait,
                    args.poll_interval,
                )
                .await
            }
            ImageSubcommand::Get(args) => {
                commands::image::get(args.task_id, args.wait, args.poll_interval).await
            }
        },
        Command::Video(command) => match command.command {
            VideoSubcommand::Generate(args) => {
                commands::video::generate(
                    args.model,
                    args.prompt,
                    args.aspect_ratio,
                    args.duration,
                    args.images,
                    args.params,
                    args.wait,
                    args.poll_interval,
                )
                .await
            }
            VideoSubcommand::Get(args) => {
                commands::video::get(args.task_id, args.wait, args.poll_interval).await
            }
        },
        Command::Task(command) => match command.command {
            TaskSubcommand::Get(args) => {
                commands::task::get(args.kind, args.task_id, args.wait, args.poll_interval).await
            }
        },
    }
}
