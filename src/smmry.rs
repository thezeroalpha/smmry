use indoc::printdoc;
use serde::{Deserialize};
use std::{env,process};

#[derive(Deserialize)]
pub struct SmmryResponse {
    sm_api_title: String,
    sm_api_content: String,
    sm_api_limitation: String
}
pub fn summarize(url: &str) -> Result<SmmryResponse, ureq::Error>{
    let api_key: String = match env::var("SMMRY_API_KEY") {
        Ok(key) => key,
        _ => {
            eprintln!("Please set the environment variable SMMRY_API_KEY to your API key for smmry.com.");
            process::exit(1);
        }
    };

    let base_url = "https://api.smmry.com/";
    let resp = ureq::get(base_url)
        .query("SM_API_KEY", &api_key)
        .query("SM_URL", url)
        .call()?;
    let data: SmmryResponse = resp.into_json()?;
    Ok(data)
}

pub enum OutputFmt { Text, Html, Markdown }

pub fn show_summary(summary: SmmryResponse, fmt: OutputFmt) {
    match fmt {
        OutputFmt::Text => {
            printdoc! {"
                {}

                {}

                Smmry: {}
                ", summary.sm_api_title, summary.sm_api_content, summary.sm_api_limitation
            };
        },
        OutputFmt::Markdown => {
            printdoc! {"
                # {}

                {}

                _Smmry: {}_
                ", summary.sm_api_title, summary.sm_api_content, summary.sm_api_limitation
            };
        },
        OutputFmt::Html => {
            printdoc! {"
                <!DOCTYPE html>
                <html>
                    <head><title>{}</title></head>
                    <body>
                    <h1>{}</h1>
                    <p>{}</p>
                    <footer>{}</footer>
                    </body>
                </html>
            ", summary.sm_api_title, summary.sm_api_title, summary.sm_api_content, summary.sm_api_limitation
            };
        },
    };
}

