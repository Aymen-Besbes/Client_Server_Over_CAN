use socketcan::{CANSocket, CANFrame };
use std::error::Error;
use std::thread;
use std::time::Duration;
mod config;
use config::load_config;


//Function to create a CAN frame within given frameid and data
fn create_frame(frame_id : u32, data : [u8;3])-> Result <CANFrame, Box<dyn Error>> {
    let frame= CANFrame ::new(frame_id, &data,false,false)?;
    println!("Frame Created successfully!");
    Ok(frame)
}

//Function to create a CAN socket
fn create_socket(interface: &str) ->Result<CANSocket,Box<dyn Error>>{
    let socket = CANSocket::open(interface)?;
    println!("Socket created successfully!");
    Ok(socket)
}



//Function that allow to send a frame within frame and socket as parampeters
fn send_frame(frame: CANFrame, socket: CANSocket)->Result<(), Box< dyn Error>>{
    loop {
        socket.write_frame(&frame)?;
        println!("Sending Frame{:?} to server .....!",&frame);
        thread::sleep(Duration::from_secs(2));
    }
}

//Test functions : 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_socket_success() {
        // Assuming "vcan0" is a valid virtual CAN interface for testing
        let result = create_socket("vcan0");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_create_socket_failure() {
        // Using an invalid interface name to simulate failure
        let result = create_socket("vcan1");
        assert!(result.is_err());
    }
    #[test]
    fn test_create_frame_success(){
        let frame =create_frame(0x123, [0xaa,0xbb,0xcc]);
        assert!(frame.is_ok());
    }
    

}
//main function
fn main()-> Result<(), Box<dyn Error>> {

    //extract the path of the configuration file from the command ""cargo run -bin server configuration_file_path" 
    let path =std::env::args().nth(1);
    
    //Get the interface , data and frame_id values from the selected configuration file
    let (interface,data,frame_id) =load_config(path)?;

    //open the socket
    let socket = create_socket(&interface)?;
    println!("Socket :::: {:?}", socket);

    //Create the frame
    let frame = create_frame(frame_id, data)?;

    //Send the data 
    send_frame(frame, socket)?;
    Ok(())
}
