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
use restson::{Error, RestClient, RestPath};
use serde_derive::{Deserialize, Serialize};
use serde_json;
use serde_json::to_string_pretty;
use std::fmt;

use super::{Institute, Status};
use super::{VscIDA, InstituteA, TimeStampA};

// ---------------------------------------------------------------
/// Command line options for account
pub fn clap_subcommand(command: &str) -> App {
    SubCommand::with_name(command)
        .arg(
            Arg::with_name("all")
                .long("all")
                .help("Get information for all accounts"),
        )
        .arg(
            Arg::with_name("institute")
                .long("institute")
                .takes_value(true)
                .help("Limit query to the given institute"),
        )
        .arg(
            Arg::with_name("institute login")
                .long("login")
                .takes_value(true)
                .help("User login at the home institute"),
        )
        .arg(
            Arg::with_name("modified")
                .long("modified")
                .takes_value(true)
                .help("Get accounts that have been modified since YYYYMMDDHHMM"),
        )
        .arg(
            Arg::with_name("vscid")
                .long("vscid")
                .takes_value(true)
                .help("The VSC id of the thing we need to fetch"),
        )
        .about("Request account information")
}

// ---------------------------------------------------------------
// data types of retrieved data

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Person {
    gecos: String,
    institute: Institute,
    institute_login: String,
    realeppn: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Account {
    vsc_id: String,
    status: Status,
    isactive: bool,
    force_active: bool,
    expiry_date: Option<String>,
    grace_until: Option<String>,
    vsc_id_number: u64,
    home_directory: String,
    data_directory: String,
    scratch_directory: String,
    login_shell: String,
    broken: bool,
    email: String,
    research_field: Vec<String>,
    create_timestamp: String,
    person: Person,
    home_on_scratch: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Accounts(pub Vec<Account>);

// ---------------------------------------------------------------
// data types for argument specification



struct InstituteLoginA(String);
impl fmt::Display for InstituteLoginA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------
// API calls

/// Retrieve all accounts
impl RestPath<()> for Accounts {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from(format!("django/api/account/")))
    }
}

/// Retrieve all accounts modified since a given timestamp
impl RestPath<&TimeStampA> for Accounts {
    fn get_path(timestamp: &TimeStampA) -> Result<String, Error> {
        Ok(String::from(format!(
            "django/api/account/modified/{}",
            timestamp
        )))
    }
}

/// Retrieve an account with the given VSC ID
impl RestPath<&VscIDA> for Account {
    fn get_path(vsc_id: &VscIDA) -> Result<String, Error> {
        Ok(String::from(format!("django/api/account/{}/", vsc_id)))
    }
}

/// Retrieve an account with the given institute and institute login
impl RestPath<(&InstituteA, &InstituteLoginA)> for Account {
    fn get_path((institute, institute_id): (&InstituteA, &InstituteLoginA)) -> Result<String, Error> {
        Ok(String::from(format!(
            "django/api/account/institute/{}/id/{}",
            institute, institute_id
        )))
    }
}

/// Process the command options and retirieve the data accordingly
pub fn process_account(
    client: &mut RestClient,
    matches: &ArgMatches,
) -> Result<String, serde_json::error::Error> {
    if matches.is_present("all") {
        let accounts: Accounts = client.get(()).unwrap();
        return to_string_pretty(&accounts);
    }

    if let Some(institute) = matches.value_of("institute") {
        if let Some(login) = matches.value_of("institute login") {
            let account: Account = client.get((&InstituteA(institute.to_string()), &InstituteLoginA(login.to_string()))).unwrap();
            return to_string_pretty(&account);
        }
    }

    if let Some(timestamp) = matches.value_of("modified") {
        let accounts: Accounts = client.get(&TimeStampA(timestamp.to_string())).unwrap();
        return to_string_pretty(&accounts);
    }

    let vsc_id = matches
        .value_of("vscid")
        .expect("You should provide a vsc id if not getting non-specific account info");
    let account: Account = client.get(&VscIDA(vsc_id.to_string())).unwrap();
    to_string_pretty(&account)
}
