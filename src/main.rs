#![feature(lookup_host)]

use std::io;
use std::net::TcpStream;
use std::time::Duration;
use std::thread;
use std::sync::{RwLock, Arc};

mod client;
use client::*;

fn main() {
	let addr = lookup_addr("who");
	let stream = TcpStream::connect(addr.as_str()).unwrap();
	stream.set_read_timeout(Some(Duration::from_millis(50))).unwrap();
	stream.set_write_timeout(Some(Duration::from_secs(1))).unwrap();

	let stream_ref = RwLock::new(stream);
	let stream_arc = Arc::new(stream_ref);

	println!("Connected!");
	let local_arc = stream_arc.clone();

	//pass("asdf", &mut local_arc.write().unwrap());
	nick(Some("cloin"), &mut local_arc.write().unwrap());
	user(Some("guest 0 * :Colin"), &mut local_arc.write().unwrap());

	let stream_lock = stream_arc.clone();
	thread::spawn(move || {
		loop {
			let reader = stream_lock.read().unwrap();
			read_response(&reader);
		}
	});

	let mut client = Client::new();
	loop {
		let mut input = String::new();
		let res = io::stdin().read_line(&mut input).ok();
		if res.is_some() {
			let input_v: Vec<&str> = input.split_whitespace().collect();
			let t_input_v = input_v.clone();
			let tmp_input = t_input_v.get(0);
			let optional = t_input_v.get(1);
			let option;
			if optional.is_some() {
				option = Some(*optional.unwrap());
			} else {
				option = None;
			}
			let input_c;
			if tmp_input.is_some() {
				input_c = *tmp_input.unwrap();
				if input_c.chars().nth(0).unwrap() == '/' {
					match input_c {
						"/pass" => { pass(option, &mut local_arc.write().unwrap()); },
						"/quit" => { quit(option, &mut local_arc.write().unwrap()); break; },
						"/ping" => { ping(option, &mut local_arc.write().unwrap()); },
						"/who" => { who(option, &mut local_arc.write().unwrap()); },
						"/nick" => { nick(option, &mut local_arc.write().unwrap()); },
						"/join" => { join(option, &mut client, &mut local_arc.write().unwrap()); },
						"/time" => { time(option, &mut local_arc.write().unwrap()); },
						"/list" => { list(option, &mut local_arc.write().unwrap()); },
						"/set" => { client.set_channel(option); },
						_ => { println!("unrecognized command: {:?}", input_c); },
					}
				} else {
					let channel = client.cur_channel.clone();
					if channel.is_some() {
						let channel = channel.clone();
						let channel_t = channel.unwrap();
						priv_msg(Some(channel_t.as_str()), Some(input.as_str()), &mut local_arc.write().unwrap());
					} else {
						println!("Channel not yet selected!");
					}
				}
			} else {
				println!("malformed input!");
			}

		} else {
			println!("invalid input!");
		}
	}
}
