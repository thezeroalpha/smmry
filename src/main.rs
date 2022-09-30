mod smmry;
use ureq::{self,Error};
use clap::{ArgGroup,Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[clap(group(
        ArgGroup::new("outfmt")
        .args(&["markdown", "text", "html"]),
))]
struct Cli {
    /// URL to summarize
    url: String,

    /// Output Markdown instead of HTML
    #[arg(short, long)]
    markdown: bool,
    /// Output plaintext instead of HTML
    #[arg(short, long)]
    text: bool,
    /// Output HTML (the default)
    #[arg(short = 'H', long, default_value_t = true)]
    html: bool,
}


fn main() {
    let cli = Cli::parse();
    let outfmt =
        if cli.markdown {
            smmry::OutputFmt::Markdown
        } else if cli.text {
            smmry::OutputFmt::Text
        } else if cli.html {
            smmry::OutputFmt::Html
        } else {
            unreachable!("Output format not set");
        };
    let data = smmry::summarize(&cli.url);
    match data {
        Ok(resp) => smmry::show_summary(resp, outfmt),
        Err(Error::Status(code, response)) => {
            println!("{} {}", code, response.status_text())
        },
        Err(e) => {
            println!("Error: {:?}", e)
        }
    };

}
