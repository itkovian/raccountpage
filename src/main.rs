use dotenv::dotenv;

use hyper::client::Client;
use hyper_tls::HttpsConnector;
use restson::{RestClient,RestPath,Error};
use serde_derive::{Deserialize, Serialize};

#[cfg(not(test))]
fn get_client() -> RestClient {

    let api_url = dotenv::var("API_URL").unwrap();

    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    println!("Using the URL {}", &api_url);


    let mut restclient = RestClient::builder()
        .with_client(client)
        .build(&api_url)
        .unwrap();

    restclient.set_header("Authorization", "Bearer ").unwrap();
    restclient.set_header("Content-Type", "application/json").unwrap();

    restclient
}

#[cfg(test)]
fn get_client() -> RestClient {
    let api_url = dotenv::var("API_URL_TEST").unwrap();
    RestClient::new(&api_url).unwrap()
}
#[derive(Serialize,Deserialize, Debug)]
struct Institute {
    name: String
}

#[derive(Serialize,Deserialize, Debug)]
struct Person {
    gecos: String,
    institute: Institute,
    institute_login: String,
    realeppn: String,
}

#[derive(Serialize,Deserialize,Debug)]
struct Account {
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
    home_on_scratch: bool
}

impl RestPath<&str> for Account {
    fn get_path(vsc_id: &str) -> Result<String, Error> { Ok(String::from(format!("django/api/account/{}/", vsc_id))) }
}

fn main() {
    dotenv().ok();

    let mut client = get_client();

    let account : Account = client.get("vsc40075").unwrap();

    println!("Account: {:?}", account);
}
