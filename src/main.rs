#![feature(lookup_host)]

mod client;

extern crate term;

use std::io;
use std::thread;

use client::*;

fn main() {
	println!("Where should I connect to?");
	let mut input = String::new();
	let res = io::stdin().read_line(&mut input).ok();

	let address_name;
	if res.is_some() {
		address_name = String::from(input.trim());
	} else {
		address_name = String::from("0.0.0.0:6667");
	}

	let mut client = Client::new(&address_name);

	let conn_arc = client.connection.clone();
	let buffer_arc = client.view_buffer.clone();
	// Poll for server updates
	thread::spawn(move || {
		loop {
			let mut conn = conn_arc.write().unwrap();
			let mut buffer = buffer_arc.write().unwrap();
			Client::handle_response(&mut conn, &mut buffer);
		}
	});

	let stdin = io::stdin();
	loop {
		let mut input = String::new();
		let res = stdin.read_line(&mut input).ok();
		if res.is_some() {
			let line = input.trim();
			if line.contains("/") {
				let tmp_line = String::from(line);
				let parts: Vec<&str> = tmp_line.split_whitespace().collect();
				let line = line.to_uppercase();
				if line == "/QUIT" {
					client.send_command("QUIT", None);
					println!("Quitting!");
					return;
				} else if line.contains("/SET") {
					println!("SETTING");
					let tmp_channel = parts.get(1);
					if tmp_channel.is_some() {
						let channel = String::from(*tmp_channel.unwrap());
						client.set_channel(Some(channel));
					} else {
						println!("Channel not yet set!");
					}
				} else if line.contains("/JOIN") {
					let tmp_channel = parts.get(1);
					if tmp_channel.is_some() {
						let channel = String::from(*tmp_channel.unwrap());
						let channel_clone = channel.clone();
						client.set_channel(Some(channel));
						client.send_command("JOIN", Some(channel_clone));
					} else {
						println!("Channel not yet set!");
					}
				} else if line.contains("/PART") {
					let tmp_channel = parts.get(1);
					if tmp_channel.is_some() {
						let channel = String::from(*tmp_channel.unwrap());
						let channel_clone = channel.clone();
						client.set_channel(None);
						client.send_command("PART", Some(channel_clone));
					} else {
						println!("Channel not yet set!");
					}
				} else {
					let command = parts[0].trim_left_matches("/");
					let mut tmp_parts = parts.clone();
					tmp_parts.remove(0);
					if parts.len() > 0 {
						let mut new_line = String::new();
						for part in tmp_parts.iter() {
							new_line.push_str(part);
							new_line.push(' ');
						}
						client.send_command(command, Some(new_line));
					} else {
						client.send_command(command, None);
					}
				}
			} else {
				if client.cur_channel.is_some() {
					client.say(&line);
				} else {
					println!("Need to set a channel first!");
				}
			}
		}
	}
}
