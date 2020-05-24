/*
Copyright 2019 Andy Georges <itkovian+raccountpage@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use clap::{App, Arg, ArgMatches, SubCommand};
use dotenv::dotenv;
use hyper::client::Client;
use hyper_tls::HttpsConnector;
use log::{error, info, LevelFilter};
use restson::{Error, RestClient};
use std::path::PathBuf;

mod entities;

use entities::account;
use entities::vo;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(not(test))]
fn get_client(token: &str) -> RestClient {
    let api_url = dotenv::var("API_URL").unwrap();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut restclient = RestClient::builder()
        .with_client(client)
        .build(&api_url)
        .unwrap();

    restclient
        .set_header("Authorization", &format!("Bearer {}", token))
        .unwrap();
    restclient
        .set_header("Content-Type", "application/json")
        .unwrap();

    restclient
}

#[cfg(test)]
fn get_client(_: &str) -> RestClient {
    let api_url = dotenv::var("API_URL_TEST").unwrap();
    RestClient::new(&api_url).unwrap()
}

fn setup_logging(debug: bool, logfile: Option<&str>) -> Result<(), log::SetLoggerError> {
    let level_filter = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    let base_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().to_rfc3339(),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(level_filter);

    match logfile {
        Some(filename) => {
            let r = fern::log_file(&PathBuf::from(filename)).unwrap();
            base_config.chain(r)
        }
        None => base_config.chain(std::io::stdout()),
    }
    .apply()
}

fn args<'a>() -> ArgMatches<'a> {
    let matches = App::new("RAccountpage")
        .version(VERSION)
        .author("Andy Georges <itkovian+raccountpage@gmail.com>")
        .about("CLI for chatting to the VSC REST API.")
        .arg(
            Arg::with_name("debug")
                .long("debug")
                .help("Log at DEBUG level."),
        )
        .arg(
            Arg::with_name("token")
                .long("token")
                .takes_value(true)
                .help("Ouath Bearer Token"),
        )
        .subcommand(account::clap_subcommand("account"))
        .subcommand(vo::clap_subcommand("vo"));

    matches.get_matches()
}

fn main() -> Result<(), Error> {
    dotenv().ok();
    let matches = args();
    match setup_logging(matches.is_present("debug"), None) {
        Ok(_) => (),
        Err(e) => panic!("Cannot set up logging: {:?}", e),
    };
    let token = matches.value_of("token").unwrap();
    let mut client = get_client(token);

    let result = match matches.subcommand() {
        ("account", Some(command_matches)) => {
            account::process_account(&mut client, command_matches)
        }
        ("vo", Some(command_matches)) => vo::process_vo(&mut client, command_matches),
        _ => Ok(String::from("oops")),
    };

    match result {
        Ok(v) => println!("{}", v),
        _ => println!("bummer"),
    }
    Ok(())
}
