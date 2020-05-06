/*
Copyright 2019 Andy Georges <itkovian+sarchive@gmail.com>

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

use clap::{ArgMatches};
use restson::{Error, RestClient, RestPath};
use serde_derive::{Deserialize, Serialize};
use serde_json;
use serde_json::to_string_pretty;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Institute {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    gecos: String,
    institute: Institute,
    institute_login: String,
    realeppn: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    vsc_id: String,
    status: String,
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

#[derive(Serialize,Deserialize,Debug,Clone)]
struct Accounts ( pub Vec<Account> );

impl RestPath<()> for Accounts {
    fn get_path(_: ()) -> Result<String, Error> {
        Ok(String::from(format!("django/api/account/")))
    }
}

impl RestPath<&str> for Accounts {
    fn get_path(timestamp: &str) -> Result<String, Error> {
        Ok(String::from(format!("django/api/account/modified/{}", timestamp)))
    }
}

impl RestPath<&str> for Account {
    fn get_path(vsc_id: &str) -> Result<String, Error> {
        Ok(String::from(format!("django/api/account/{}/", vsc_id)))
    }
}

pub fn process_account(client: &mut RestClient, matches: &ArgMatches) -> Result<String, serde_json::error::Error> {
    if matches.is_present("all") {
        let accounts : Accounts = client.get(()).unwrap();
        to_string_pretty(&accounts)
    }
    else if let Some(timestamp) = matches.value_of("modified") {
        let accounts : Accounts = client.get(timestamp).unwrap();
        //println!("Accounts: {:?}", accounts);
        to_string_pretty(&accounts)
    }
    else {
        let vsc_id = matches.value_of("vscid").expect("You should provide a vsc id if not getting all account info");
        let account: Account = client.get(vsc_id).unwrap();
        //println!("Account: {:?}", account);
        to_string_pretty(&account)
    }
}
