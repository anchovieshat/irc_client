use std::io::Write;
use std::io::BufRead;
use std::io::BufReader;
use std::net;
use std::net::TcpStream;
use std::sync::{RwLock, Arc};
use std::time::Duration;

pub fn lookup_addr(site_name: &str) -> String {
	let table = net::lookup_host(site_name).ok();
	let mut addr;
	if !table.is_some() {
		addr = String::from("0.0.0.0:6667");
		println!("invalid address: {}", site_name);
		println!("using default: {}", addr);
		return addr;
	}

	addr = format!("{}", table.unwrap().nth(0).unwrap().unwrap());
	let size = addr.len();
	addr.truncate(size - 2);
	addr.push_str(":6667");
	println!("Attempting connection for {}: {}", site_name, addr);
	return addr;
}

pub fn irc_command(command: &str, options: Option<&str>) -> String {
	let mut send = String::new();
	send.push_str(command);
	if options.is_some() {
		send.push(' ');
		send.push_str(options.unwrap());
	}
	send = String::from(send.trim());
	send.push_str("\r\n");
	send
}

pub struct Client {
	pub connection: Arc<RwLock<TcpStream>>,
	pub view_buffer: Arc<RwLock<Vec<String>>>,
	pub nick: String,
	pub password: String,
	pub cur_channel: Option<String>,
}

impl Client {
	pub fn new(address: &String) -> Client {
		let valid_conn = TcpStream::connect(lookup_addr(address).as_str()).ok();
		let mut stream;
		if valid_conn.is_some() {
			stream = valid_conn.unwrap();
		} else {
			panic!("Connection to {} failed", address);
		}
		stream.set_read_timeout(Some(Duration::from_millis(500))).unwrap();
		stream.set_write_timeout(Some(Duration::from_secs(1))).unwrap();

		let nick = "cloin";
		let password = "a";

		stream.write(irc_command("NICK", Some(nick)).as_bytes()).unwrap();
		stream.write(irc_command("USER", Some("guest 0 * :Colin")).as_bytes()).unwrap();

		Client {
			connection: Arc::new(RwLock::new(stream)),
			view_buffer: Arc::new(RwLock::new(Vec::new())),
			nick: String::from(nick),
			password: String::from(password),
			cur_channel: None,
		}
	}

	pub fn set_channel(&mut self, channel: &String) {
		self.cur_channel = Some(channel.clone());
	}

	pub fn say(&mut self, message: &str) {
		let channel = self.cur_channel.clone();
		self.priv_msg(channel.unwrap().as_str(), message);
	}

	pub fn priv_msg(&mut self, target: &str, message: &str) {
		let mut res = String::new();
		res.push_str(target);
		res.push_str(" :");
		res.push_str(message);

		let stream_arc = self.connection.clone();
		let mut stream = stream_arc.write().unwrap();
		stream.write(irc_command("PRIVMSG", Some(res.as_str())).as_bytes()).unwrap();
	}

	pub fn send_command(&mut self, command: &str, options: Option<String>) {
		let mut res = String::new();

		if options.is_some() {
			res = String::from(options.unwrap());
		}

		let stream_arc = self.connection.clone();
		let mut stream = stream_arc.write().unwrap();
		let command_string = irc_command(command, Some(res.as_str()));
		stream.write(command_string.as_bytes()).unwrap();
	}

	pub fn handle_response(conn: &mut TcpStream, buffer: &mut Vec<String>) {
		let r = BufReader::new(conn);
		for line in r.lines() {
			let line = line.ok();
			if line.is_some() {
				let line = line.unwrap();
				println!("{}", line);
				buffer.push(line);
			} else {
				break;
			}
		}
	}
}
