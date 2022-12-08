use clap::Parser;
use std::env;
use anyhow::{Result};
use serde::{ Deserialize, Serialize };

use tungstenite::{connect, Message};
use url::Url;
use serde_json;
 
#[derive(Clone, Debug, Parser)]
pub struct Start {  
    /// Specify this as a prover, with the given account private key for this node.
    #[clap(long = "prover")]
    pub prover: Option<String>,    
    /// If the flag is set, the node will not render the display.
    #[clap(long)]
    pub nodisplay: bool,
    
}

#[derive(    
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
)]
pub struct EpochChallenge {
	epoch_number: u64,
	epoch_block_hash: String,
	degree: u64,
}

#[derive(    
    Serialize,
    Deserialize,
    Clone,
    Debug,
    PartialEq,
)]
pub struct MessageToServer {
	_type: String,
	epoch_challenge: EpochChallenge,
	proof_target: u64,
	block_height: u64,	
}

impl Start {
    /// Starts the snarkOS node.
    pub fn parse(self) -> Result<String> {
        println!( "⚠️  Running" );
                
		let pool_ws_address = env::var("POOL_WS_ADDRESS").expect("wrong address");
		
		println!("pool_ws_address {} ", pool_ws_address);
		
		let epoch_challenge = EpochChallenge {
			epoch_number: 4083006752,
			epoch_block_hash: "ab1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq5g436j".to_owned(),
			degree:8191,
		};
		
		let message_to_server = MessageToServer {
			_type: "job".to_owned(),
			epoch_challenge: epoch_challenge,
			proof_target: 300000,
			block_height: 100,
		};
		
		let message_to_server = serde_json::to_string(&message_to_server)?;
		
		println!("message_to_server {} ", message_to_server);
		
		let (mut socket, _response) = connect(Url::parse(&pool_ws_address).unwrap()).expect("Can't connect"); 
		
		socket.write_message(Message::Text(message_to_server)).unwrap();
		
		// Loop forever, handling parsing each message
		loop {
			let msg = socket.read_message().expect("Error reading message");
			let msg = match msg {
				tungstenite::Message::Text(s) => { s }
				_ => { panic!() }
			};
			let parsed: serde_json::Value = serde_json::from_str(&msg).expect("Can't parse to JSON");
			println!("{:?}", parsed["result"]);
		}		

        Ok(String::new())
    }
}

