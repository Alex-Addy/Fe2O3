#![feature(net)]
use std::net;

fn main() {
	let res = net::lookup_host("irc.slashnet.org");

	for host in res.unwrap() {
		println!("found address: {}", host.unwrap());
	}
}
