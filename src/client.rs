use std::io::Write;
use std::io::BufRead;
use std::io::BufReader;
use std::net;
use std::net::TcpStream;

pub fn lookup_addr(site_name: &str) -> String {
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
pub fn read_response(stream: &TcpStream) {
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

pub fn irc_command(command: &str, options: Option<&str>) -> String {
	let mut send = String::new();
	send.push_str(command);
	send.push(' ');
	if options.is_some() {
		send.push_str(options.unwrap());
	}
	send.push_str("\r\n");
	send
}

pub fn quit(why: Option<&str>, stream: &mut TcpStream) {
	if why.is_some() {
		let mut fmt_why = String::from(":");
		fmt_why.push_str(why.unwrap());
		stream.write(irc_command("QUIT", Some(why.unwrap())).as_bytes()).unwrap();
	} else {
		stream.write(irc_command("QUIT", None).as_bytes()).unwrap();
	}
}

pub fn ping(addr: Option<&str>, stream: &mut TcpStream) {
	if addr.is_some() {
		let addr = addr.unwrap();
		stream.write(irc_command("PING", Some(addr)).as_bytes()).unwrap();
	} else {
		stream.write(irc_command("PING", Some("0.0.0.0")).as_bytes()).unwrap();
	}
}

pub fn who(specifier: Option<&str>, stream: &mut TcpStream) {
	if specifier.is_some() {
		let specifier = specifier.unwrap();
		stream.write(irc_command("WHO", Some(specifier)).as_bytes()).unwrap();
	} else {
		stream.write(irc_command("WHO", None).as_bytes()).unwrap();
	}
}

pub fn nick(nick: Option<&str>, stream: &mut TcpStream) {
	if nick.is_some() {
		let nick = nick.unwrap();
		stream.write(irc_command("NICK", Some(nick)).as_bytes()).unwrap();
	} else {
		println!("No nick specified!");
	}
}

pub fn join(channel: Option<&str>, client: &mut Client, stream: &mut TcpStream) {
	if channel.is_some() {
		let channel = channel.unwrap();
		client.set_channel(Some(channel));
		stream.write(irc_command("JOIN", Some(channel)).as_bytes()).unwrap();
	} else {
		println!("No channel specified!");
	}
}

pub fn pass(pass: &str, stream: &mut TcpStream) {
	stream.write(irc_command("PASS", Some(pass)).as_bytes()).unwrap();
}

pub fn user(user_type: &str, stream: &mut TcpStream) {
	stream.write(irc_command("USER", Some(user_type)).as_bytes()).unwrap();
}

pub fn time(addr: Option<&str>, stream: &mut TcpStream) {
	if addr.is_some() {
		let addr = addr.unwrap();
		stream.write(irc_command("TIME", Some(addr)).as_bytes()).unwrap();
	} else {
		stream.write(irc_command("TIME", None).as_bytes()).unwrap();
	}
}

pub fn list(channel: Option<&str>, stream: &mut TcpStream) {
	if channel.is_some() {
		let channel = channel.unwrap();
		stream.write(irc_command("LIST", Some(channel)).as_bytes()).unwrap();
	} else {
		stream.write(irc_command("LIST", None).as_bytes()).unwrap();
	}
}

pub fn priv_msg(target: Option<&str>, message: Option<&str>, stream: &mut TcpStream) {
	if target.is_some() && message.is_some() {
		let target = target.unwrap();
		let message = message.unwrap();
		let mut res = String::new();
		res.push_str(target);
		res.push_str(" :");
		res.push_str(message);
		stream.write(irc_command("PRIVMSG", Some(res.as_str())).as_bytes()).unwrap();
	} else {
		println!("message malformed! {:?} {:?}", target, message);
	}
}


pub struct Client {
	pub cur_channel: Option<String>,
}

impl Client {
	pub fn new() -> Client {
		Client {
			cur_channel: None,
		}
	}

	pub fn set_channel(&mut self, channel: Option<&str>) {
		if channel.is_some() {
			let channel = String::from(channel.unwrap());
			self.cur_channel = Some(channel);
		} else {
			println!("You didn't give me a channel to set...");
		}
	}


}
