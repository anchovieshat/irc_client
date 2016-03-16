#![feature(lookup_host)]

use std::io;
use std::net::TcpStream;
use std::time::Duration;
use std::thread;
use std::sync::{RwLock, Arc};

mod client;
use client::*;

fn main() {
	println!("+---------+");
	println!("| IRC FUN |");
	println!("+---------+");
	println!("Where should I connect to?");
	let mut input = String::new();
	let res = io::stdin().read_line(&mut input).ok();

	let address_name;
	if res.is_some() {
		address_name = input.trim();
	} else {
		address_name = "0.0.0.0:6667";
	}

	let addr = lookup_addr(address_name);
	let valid_conn = TcpStream::connect(addr.as_str()).ok();
	let stream;
	if valid_conn.is_some() {
		stream = valid_conn.unwrap();
		println!("Address verified!");
	} else {
		println!("Connection to {} failed", addr);
		return;
	}
	stream.set_read_timeout(Some(Duration::from_millis(500))).unwrap();
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
			let mut handler = stream_lock.write().unwrap();
			read_response(&mut handler);
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
						"/quit" => {
							let mut input = String::new();
							let mut input_v = input_v.clone();
							input_v.remove(0);
							for string in input_v.iter() {
								input.push_str(string);
								input.push(' ');
							}
							quit(Some(input.as_str()), &mut local_arc.write().unwrap()); break;
						},
						"/ping" => { ping(option, &mut local_arc.write().unwrap()); },
						"/pong" => { pong(option, &mut local_arc.write().unwrap()); },
						"/who" => { who(option, &mut local_arc.write().unwrap()); },
						"/nick" => { nick(option, &mut local_arc.write().unwrap()); },
						"/join" => { join(option, &mut client, &mut local_arc.write().unwrap()); },
						"/time" => { time(option, &mut local_arc.write().unwrap()); },
						"/list" => { list(option, &mut local_arc.write().unwrap()); },
						"/pm" => {
							let mut input = String::new();
							let mut input_v = input_v.clone();
							input_v.remove(0);
							input_v.remove(0);
							for string in input_v.iter() {
								input.push_str(string);
								input.push(' ');
							}
							priv_msg(option, Some(input.as_str()), &mut local_arc.write().unwrap());
						},
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
