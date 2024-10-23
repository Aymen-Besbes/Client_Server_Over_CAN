//imports
use socketcan::CANSocket;
use std::error::Error;
use std::io;
use std::process::Command;
mod config;
use config::load_config;

//Function that allow to run a shell script to create a virtual can
fn create_virtual_can()->io:: Result<()> {
  let output = Command::new("VirCanCreation.sh").output()?;
  if output.status.success() {
    println!("Script executed successfully");
  } else {
    eprintln!("Script execution failed");
  }
  Ok(())

}

//Function that allows to create a socket
fn create_socket(interface: &str) ->Result<CANSocket,Box<dyn Error>>{
  let socket = CANSocket::open(interface)?;
  println!("Socket created successfully!");
  Ok(socket)

}

//Function that allows to receive CAN frame
fn receive_frame(socket: CANSocket)->Result<(), Box< dyn Error>>{
  loop {
    match socket.read_frame(){
        Ok(frame)=> {
            println!("Rceiving CAN frame : {:?} ......!",frame);
          },
        Err(e) => {
          eprintln!("Failed to read CAN:{}",e);
        }
    }
}
}

//main function
fn main()-> Result<(), Box<dyn Error>> {
  
  //create a virtual can
  let _=create_virtual_can();

  //extract the path of the configuration file from the command ""cargo run -bin server configuration_file_path" 
  let path =std::env::args().nth(1);

  //Get the interface value from the selected configuration file
  let (interface,_,_) =load_config(path)?;

  //Open the socket
  let socket = create_socket(&interface)?;

  //Receiving the data 
  let _=receive_frame(socket);

  Ok(())
}

