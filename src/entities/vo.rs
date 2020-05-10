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
pub struct VirtualOrganisation {
    vsc_id: String,
    status: Status,
    vsc_id_number: u64,
    institute: Institute,
    //name: String,  # this is mentioned in the API docs :/
    fairshare: u32,
    data_path: String,
    scratch_path: String,
    description: String,
    members: Vec<String>,
    moderators: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct VirtualOrganisations(pub Vec<VirtualOrganisation>);

// ---------------------------------------------------------------
// API calls

/// Retrieve all vos
impl RestPath<()> for VirtualOrganisations {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from(format!("django/api/vo/")))
    }
}

/// Retrieve a single VO
impl RestPath<&VscIDA> for VirtualOrganisation {
    fn get_path(vscid: &VscIDA) -> Result<String, Error> {
        Ok(String::from(format!("django/api/vo/{}", vscid)))
    }
}


/// Process the command options and retirieve the data accordingly
pub fn process_vo(
    client: &mut RestClient,
    matches: &ArgMatches,
) -> Result<String, serde_json::error::Error> {

    if matches.is_present("all") {
        let vos: VirtualOrganisations = client.get(()).unwrap();
        return to_string_pretty(&vos);
    }

    let vsc_id = matches
        .value_of("vscid")
        .expect("You should provide a vsc id if not getting non-specific account info");
    let vo : VirtualOrganisation = client.get(&VscIDA(vsc_id.to_string())).unwrap();
    return to_string_pretty(&vo);
}