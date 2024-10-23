use serde::{Serialize, Deserialize};
use std::fs;
use std::string::String;
use std::error::Error as BoxError;
use serde_json ;

//define config struct
#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    client_app: Option<ConfigClient>,
    server_app: Option<ConfigServer>,
}

//Definine client inputs
#[derive(Serialize, Deserialize, Debug)]
struct ConfigClient{
    interface: Option<String>,
    data: Option<[u8; 3]>,
    frame_id: Option<u32>,
}

//Definine server inputs
#[derive(Serialize, Deserialize, Debug)]
struct ConfigServer{
    interface: Option<String>,
}



//Public function to use in client and server applications to load the configuration file
pub fn load_config(path_config: Option<String>) ->Result< (String, [u8;3],u32), Box<dyn BoxError>>{

    //set the default path for the configuration
    let default: String =String::from("src/config/default.json");
    
    //set the path : the mentioned path in args if exists else the default path
    
    let path_config :String = path_config.unwrap_or_else(|| default);
    //Read the configuration file given the path
    let content = fs::read_to_string(&path_config)?;
    
    let config: ConfigToml = serde_json::from_str(&content)?;

    //set default values for missing client inputs
    let default_client = ConfigClient {
        interface: Some(String::from("vcan0")),
        data: Some([0, 0, 0]),
        frame_id: Some(0x123),
    };

    //set default values for missing server inputs
    let default_server = ConfigServer {
        interface: Some(String::from("vcan0")),
    };

    //save the configuration fields into variables 
    let client_interface = config.client_app.as_ref()
        .and_then(|client| client.interface.clone())
        .unwrap_or(default_client.interface.unwrap());
    let client_data = config.client_app.as_ref()
        .and_then(|client| client.data)
        .unwrap_or(default_client.data.unwrap());
    let client_frame_id = config.client_app.as_ref()
        .and_then(|client| client.frame_id)
        .unwrap_or(default_client.frame_id.unwrap());
    let server_interface = config.server_app.as_ref()
        .and_then(|server| server.interface.clone())
        .unwrap_or(default_server.interface.unwrap());

    //return configuration fields
    Ok((client_interface, client_data, client_frame_id))
    
    
}


//main function to test the utility of the code 
fn main() -> Result<(), Box<dyn BoxError >>{
    let mut path : Option<String> = Some(String::from("src/config/default.json")) ;
    let config = load_config(path)?;
    println!("config.interface: {:?}",config);
    Ok(())
}

