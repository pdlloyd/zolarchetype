use serde::Serialize; // 1.0.91
use std::{collections::BTreeMap, fs};
use toml; // 0.5.1

#[derive(Debug, Default, Serialize)]
struct Servers<'a> {
    servers: BTreeMap<&'a str, Server<'a>>,
}

#[derive(Debug, Serialize)]
struct Server<'a> {
    #[serde(rename = "Ipaddr")]
    ip_addr: &'a str,

    #[serde(rename = "Port no")]
    port_no: i64,
}

fn main() {
    let mut file = Servers::default();
    file.servers.insert(
        "A",
        Server {
            ip_addr: "192.168.4.1",
            port_no: 4476,
        },
    );
    file.servers.insert(
        "B",
        Server {
            ip_addr: "192.168.4.8",
            port_no: 1234,
        },
    );

    let toml_string = toml::to_string(&file).expect("Could not encode TOML value");
    println!("{}", toml_string);
    fs::write("servers.toml", toml_string).expect("Could not write to file!");
}