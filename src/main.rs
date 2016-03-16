#![feature(lookup_host)]

use std::io::prelude::*;
use std::io::BufReader;
use std::io;
use std::net;
use std::net::{TcpStream, TcpListener};
use std::time::Duration;
use std::thread;
use std::sync::{RwLock, Arc};

fn read_response(stream: &TcpStream) {
	let mut r = BufReader::new(stream);
	for line in r.lines() {
		let line = line.ok();
		if line.is_some() {
			println!("{}", line.unwrap());
		} else {
			break;
		}
	}
}

fn irc_command(command: &str, options: Option<&str>) -> String {
	let mut send = String::new();
	send.push_str(command);
	send.push(' ');
	if options.is_some() {
		send.push_str(options.unwrap());
	}
	send.push_str("\r\n");
	send
}

fn quit(why: Option<&str>, stream: &mut TcpStream) {
	if why.is_some() {
		let mut fmt_why = String::from(":");
		fmt_why.push_str(why.unwrap());
		stream.write(irc_command("QUIT", Some(fmt_why.as_str())).as_bytes()).unwrap();
	} else {
		stream.write(irc_command("QUIT", None).as_bytes()).unwrap();
	}
}

fn ping(stream: &mut TcpStream) {
	stream.write(irc_command("PING", Some("0.0.0.0")).as_bytes()).unwrap();
}

fn who(specifier: Option<&str>, stream: &mut TcpStream) {
	if specifier.is_some() {
		let specifier = specifier.unwrap();
		stream.write(irc_command("WHO", Some(specifier)).as_bytes()).unwrap();
	} else {
		stream.write(irc_command("WHO", None).as_bytes()).unwrap();
	}
}

fn nick(nick: &str, stream: &mut TcpStream) {
	stream.write(irc_command("NICK", Some(nick)).as_bytes()).unwrap();
}

fn pass(pass: &str, stream: &mut TcpStream) {
	stream.write(irc_command("PASS", Some(pass)).as_bytes()).unwrap();
}

fn user(user_type: &str, stream: &mut TcpStream) {
	stream.write(irc_command("USER", Some(user_type)).as_bytes()).unwrap();
}

fn lookup_addr(site_name: &str) -> String {
	let table = net::lookup_host(site_name).ok();
	let mut addr;
	if !table.is_some() {
		addr = String::from("0.0.0.0:6667");
		println!("invalid address: \"{}\"", site_name);
		println!("using default: {}", addr);
		return addr;
	}

	addr = format!("{}", table.unwrap().nth(0).unwrap().unwrap());
	let size = addr.len();
	addr.truncate(size - 2);
	addr.push_str(":6667");
	println!("{}: {}", site_name, addr);
	return addr;
}

fn main() {
	let mut stream = TcpStream::connect(lookup_addr("nop").as_str()).unwrap();
	stream.set_read_timeout(Some(Duration::from_secs(1))).unwrap();
	stream.set_write_timeout(Some(Duration::from_secs(1))).unwrap();

	let stream_ref = RwLock::new(stream);
	let stream_arc = Arc::new(stream_ref);
	let text_buffer: Vec<String> = Vec::new();

	println!("Connected!");
	let local_arc = stream_arc.clone();

	//pass("asdf", &mut local_arc.write().unwrap());
	nick("cloin", &mut local_arc.write().unwrap());
	user("guest 0 * :Colin", &mut local_arc.write().unwrap());

	let stream_lock = stream_arc.clone();
	let child = thread::spawn(move || {
		loop {
			let reader = stream_lock.read().unwrap();
			read_response(&reader);
		}
	});

	loop {
		let mut input = String::new();
		let res = io::stdin().read_line(&mut input).ok();
		if res.is_some() {
			let input = input.trim_matches('\n');
			match input {
				"/quit" => { quit(Some("Was sunny today!"), &mut local_arc.write().unwrap()); break; },
				"/ping" => { ping(&mut local_arc.write().unwrap()); },
				"/who" => { who(None, &mut local_arc.write().unwrap()); },
				_ => { println!("unrecognized command: {:?}", input); },
			}
		} else {
			println!("invalid input!");
		}
	}
}
