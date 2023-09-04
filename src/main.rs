use crate::valclient_rs::auth::Auth;
use std::io::stdin;
use std::process::exit;

mod valclient_rs;

fn main() {
    println!("launching test");

    let _auth = Auth::new("".to_string(), "".to_string());

    let mut region = String::new();
    println!("Enter region: ");
    stdin()
        .read_line(&mut region)
        .expect("Failed to read input");
    region = region.strip_suffix('\n').unwrap_or(&region).to_string();
    println!();
    let client_res = valclient_rs::ValClient::new(&region, None);
    let mut client = match client_res {
        Ok(client) => client,
        Err(err) => {
            println!("{:?}", err);
            exit(-1)
        }
    };

    match client.activate() {
        Ok(_) => println!("Activated client"),
        Err(err) => println!("{:?}", err),
    }

    println!("lockfile path: {}", client.lockfile.get_path_str());

    println!("{}", client.resources.get_base_local_endpoint());
    println!("{}", client.resources.get_base_endpoint());
    println!("{}", client.resources.get_base_glz_endpoint());
    println!("{}", client.resources.get_base_shared_endpoint());
}
