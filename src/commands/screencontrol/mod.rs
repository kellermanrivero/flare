use std::fs::File;
use std::io::Write;
use std::time::Instant;

use clap::{Arg, ArgMatches, Command, value_parser};

mod framebuffer;
mod network;
mod protocol;
mod encoders;

pub struct ScreenControlCommand();

pub const COMMAND_NAME: &str = "screencontrol";
const PORT_ARG: &str = "port";

impl super::FlareCommand for ScreenControlCommand {
    fn get_definition(&self) -> Command {
        return Command::new(COMMAND_NAME).arg(
            Arg::new(PORT_ARG)
                .value_parser(value_parser!(u16)));
    }

    fn execute(&self, arg_matches: &ArgMatches) {
        let port = *arg_matches.get_one::<u16>(PORT_ARG).expect("You must specify a port");
        let fb = framebuffer::get_framebuffer();
        println!("Framebuffer details:");
        println!("{}", fb);

        let client = network::UdpClient::create(port);

        // Connect
        client.connect();

        loop {
            let start = Instant::now();
            let framebuffer_data = fb.get_contents();

            if fb.bits_per_pixel == 8 {
                todo!("Do something to deal with gray scale images")
            }

            /*
            let mut file = File::create("sample.raw").unwrap();
            file.write_all(framebuffer_data).expect("Cannot write sample file");*/

            let encoder = encoders::Encoders::jpeg();
            let encoded_data = encoder.encode(framebuffer_data, fb.width, fb.height, fb.stride, fb.bits_per_pixel, 50);
            let encoded_length = encoded_data.len();

            let mut file = File::create("sample.jpeg").unwrap();
            file.write_all(&*encoded_data).expect("Cannot write sample file");

            let frame = protocol::Frame::from(fb.width, fb.height, fb.bits_per_pixel, encoded_data);
            let packets = frame.packets();
            println!("Sending {} packets... (data payload: {})", packets.len(), encoded_length);
            for packet in packets {
                client.send(packet.get_data());
            }

            let duration = start.elapsed();
            println!("Sent!!! Time elapsed is: {:?}", duration);
        }
    }
}
