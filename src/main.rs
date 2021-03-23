extern crate dotenv;

use dotenv::dotenv;
use nats;
use std::env;

fn main() {
    connect_nats();
}

fn connect_nats() {
    dotenv().ok();

    let nats_url = match env::var("NATS_ENDPOINT") {
        Ok(url) => url,
        Err(e) => panic!("{}", e),
    };

    let nats_subj = match env::var("NATS_SUBJECT") {
        Ok(subj) => subj,
        Err(e) => panic!("{}", e),
    };

    let nc = nats::connect(&nats_url);
    let sub = nc.subscribe(&nats_subj);
}
