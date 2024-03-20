use dialoguer::{theme::ColorfulTheme, Input, FuzzySelect};
use std::{thread, time, env};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

use http_body_util::Empty;
use hyper::Request;
use hyper::body::Bytes;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use http_body_util::BodyExt;
use tokio::io::{AsyncWriteExt as _, self};
use rustc_serialize::json;

#[derive(RustcEncodable, RustcDecodable)]
struct JsonRequestGetRounds {
    response: String,
}

#[derive(Debug, Deserialize, RustcEncodable)]
struct RoundCount {
    round_count: u32,
}

#[derive(RustcEncodable, RustcDecodable)]
struct JsonRequest {
    response: String,
    pk_share: u32,
    id: u32,
    round_id: u32,
}

#[derive(Debug, Deserialize, RustcEncodable)]
struct CrispConfig {
    round_id: u32,
    chain_id: u32,
    voting_address: String,
    cyphernode_count: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let selections = &[
        "CRISP: Voting Protocol (ETH)",
        "More Coming Soon!"
    ];

    let selections_2 = &[
        "Initialize new CRISP round.",
        "Continue Existing CRISP round."
    ];

    let selection_1 = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Enclave (EEEE): Please choose the private execution environment you would like to run!")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    if(selection_1 == 0){
    	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    	println!("Encrypted Protocol Selected {}!", selections[selection_1]);
	    let selection_2 = FuzzySelect::with_theme(&ColorfulTheme::default())
	        .with_prompt("Create a new CRISP round or particpate in an existing round.")
	        .default(0)
	        .items(&selections_2[..])
	        .interact()
	        .unwrap();

	    if(selection_2 == 0){
	    	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
	    	println!("Starting new CRISP round!");
		    // let input_token: String = Input::with_theme(&ColorfulTheme::default())
		    //     .with_prompt("Enter Proposal Registration Token")
		    //     .interact_text()
		    //     .unwrap();
		    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
		    println!("Reading proposal details from config.");
            let path = env::current_dir().unwrap();
            let mut pathst = path.display().to_string();
            pathst.push_str("/example_config.json");
            let mut file = File::open(pathst).unwrap();
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();
            let config: CrispConfig = serde_json::from_str(&data).expect("JSON was not well-formatted");
            println!("round id: {:?}", config.round_id); // get new round id from current id in server
            println!("chain id: {:?}", config.chain_id);
            println!("voting contract: {:?}", config.voting_address);
            println!("cyphernode count: {:?}", config.cyphernode_count);

            println!("Calling contract to initialize onchain proposal...");
	        let three_seconds = time::Duration::from_millis(1000);
	        thread::sleep(three_seconds);

            println!("Initializing Keyshare nodes...");
            // call init on server
            // have nodes poll

            // Todo: pull client code into function that takes endpoint url and body as input 
            // Client Code
            // Parse our URL for registering keyshare...
            let url_id = "http://127.0.0.1/get_rounds".parse::<hyper::Uri>()?;
            // Get the host and the port
            let host_id = url_id.host().expect("uri has no host");
            let port_id = url_id.port_u16().unwrap_or(3000);
            let address_id = format!("{}:{}", host_id, port_id);
            // Open a TCP connection to the remote host
            let stream_id = TcpStream::connect(address_id).await?;
            // Use an adapter to access something implementing `tokio::io` traits as if they implement
            // `hyper::rt` IO traits.
            let io_id = TokioIo::new(stream_id);
            // Create the Hyper client
            let (mut sender_id, conn_id) = hyper::client::conn::http1::handshake(io_id).await?;
            // Spawn a task to poll the connection, driving the HTTP state
            tokio::task::spawn(async move {
                if let Err(err) = conn_id.await {
                    println!("Connection failed: {:?}", err);
                }
            });
            // The authority of our URL will be the hostname of the httpbin remote
            let authority_id = url_id.authority().unwrap().clone();
            let response_id = JsonRequestGetRounds { response: "Test".to_string() };
            let out_id = json::encode(&response_id).unwrap();
            let req_id = Request::get("http://127.0.0.1/")
                .uri(url_id.clone())
                .header(hyper::header::HOST, authority_id.as_str())
                .body(out_id)?;
            let mut res_id = sender_id.send_request(req_id).await?;

            println!("Response status: {}", res_id.status());

            let body_bytes = res_id.collect().await?.to_bytes();
            let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
            let count: RoundCount = serde_json::from_str(&body_str).expect("JSON was not well-formatted");
            println!("Server Round Count: {:?}", count.round_count);


            // Client Code --------------------------------
            // Parse our URL for registering keyshare...
            let url = "http://127.0.0.1/init_crisp_round".parse::<hyper::Uri>()?;
            // Get the host and the port
            let host = url.host().expect("uri has no host");
            let port = url.port_u16().unwrap_or(3000);
            let address = format!("{}:{}", host, port);
            // Open a TCP connection to the remote host
            let stream = TcpStream::connect(address).await?;
            // Use an adapter to access something implementing `tokio::io` traits as if they implement
            // `hyper::rt` IO traits.
            let io = TokioIo::new(stream);
            // Create the Hyper client
            let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
            // Spawn a task to poll the connection, driving the HTTP state
            tokio::task::spawn(async move {
                if let Err(err) = conn.await {
                    println!("Connection failed: {:?}", err);
                }
            });
            // The authority of our URL will be the hostname of the httpbin remote
            let authority = url.authority().unwrap().clone();
            let round_id = count.round_count + 1;
            let response = CrispConfig { round_id: round_id, chain_id: 5, voting_address: "Test".to_string(), cyphernode_count: 3 };
            //let response = JsonRequest { response: "Test".to_string(), pk_share: 0, id: 0, round_id: 0 };
            let out = json::encode(&response).unwrap();
            let req = Request::post("http://127.0.0.1/")
                .uri(url.clone())
                .header(hyper::header::HOST, authority.as_str())
                .body(out)?;

            let mut res = sender.send_request(req).await?;

            println!("Response status: {}", res.status());

            // Stream the body, writing each frame to stdout as it arrives
            while let Some(next) = res.frame().await {
                let frame = next?;
                if let Some(chunk) = frame.data_ref() {
                    io::stdout().write_all(chunk).await?;
                }
            }
            println!("Round Initialized.");
	    	println!("Gathering Keyshare nodes for execution environment...");
            thread::sleep(three_seconds);
            println!("You can now vote Encrypted with Round ID: {:?}", round_id);

	    }
	    if(selection_2 == 1){
	    	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
		    let input_crisp_id: String = Input::with_theme(&ColorfulTheme::default())
		        .with_prompt("Enter CRISP round ID.")
		        .interact_text()
		        .unwrap();
	    }

    }
    if(selection_1 == 1){
    	println!("Check back soon!");
    	std::process::exit(1);
    }

    // println!("Hello {}!", input);

    // let mail: String = Input::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Your email")
    //     .validate_with({
    //         let mut force = None;
    //         move |input: &String| -> Result<(), &str> {
    //             if input.contains('@') || force.as_ref().map_or(false, |old| old == input) {
    //                 Ok(())
    //             } else {
    //                 force = Some(input.clone());
    //                 Err("This is not a mail address; type the same value again to force use")
    //             }
    //         }
    //     })
    //     .interact_text()
    //     .unwrap();

    // println!("Email: {}", mail);

    // let mail: String = Input::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Your planet")
    //     .default("Earth".to_string())
    //     .interact_text()
    //     .unwrap();

    // println!("Planet: {}", mail);

    // let mail: String = Input::with_theme(&ColorfulTheme::default())
    //     .with_prompt("Your galaxy")
    //     .with_initial_text("Milky Way".to_string())
    //     .interact_text()
    //     .unwrap();

    // println!("Galaxy: {}", mail);
    Ok(())
}
