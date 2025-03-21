use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a model
    Run {
        /// Name of the model to use
        model: String,

        /// Temperature for generation
        #[arg(short, long, default_value_t = 0.7)]
        temperature: f32,
    },

    /// Serve a model over http
    Serve {},
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { model, temperature } => {
            todo!();
        }

        Commands::Serve {} => {
            todo!();
        }
    }

    Ok(())
}
