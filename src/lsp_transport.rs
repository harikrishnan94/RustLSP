// Copyright 2016 Bruno Medeiros
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0>.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::io::{self, Read};

use util::core::*;

use jsonrpc::service_util::MessageReader;
use jsonrpc::service_util::MessageWriter;

/* -----------------  ----------------- */

pub struct LSPMessageReader<T: io::BufRead>(pub T);

impl<T: io::BufRead> MessageReader for LSPMessageReader<T> {
	fn read_next(&mut self) -> GResult<String> {
		parse_transport_message(&mut self.0)
	}
}

pub struct LSPMessageWriter<T: io::Write>(pub T);

impl<T: io::Write> MessageWriter for LSPMessageWriter<T> {
	fn write_message(&mut self, msg: &str) -> Result<(), GError> {
		write_transport_message(msg, &mut self.0)
	}
}

/* ----------------- Parse content-length ----------------- */

const CONTENT_LENGTH: &'static str = "Content-Length:";

pub fn parse_transport_message<R: io::BufRead + ?Sized>(reader: &mut R) -> GResult<String> {
	let mut content_length: u32 = 0;

	loop {
		let mut line = String::new();

		try!(reader.read_line(&mut line));

		if line.starts_with(CONTENT_LENGTH) {
			let len_str: &str = &line[CONTENT_LENGTH.len()..];
			let int_result = len_str.trim().parse::<u32>();

			content_length = try!(int_result);
		} else if line.eq("\r\n") {
			break;
		} else if line.is_empty() {
			return Err("End of stream reached.".into());
		}
	}
	if content_length == 0 {
		return Err((String::from(CONTENT_LENGTH) + " not defined or invalid.").into());
	}

	let mut message_reader = reader.take(content_length as u64);
	let mut message = String::new();
	try!(message_reader.read_to_string(&mut message));
	return Ok(message);
}

pub fn write_transport_message<WRITE: io::Write>(message: &str, out: &mut WRITE) -> GResult<()> {
	//    let out : &mut io::Write = out;
	try!(out.write_all(CONTENT_LENGTH.as_bytes()));
	try!(out.write(&[' ' as u8]));
	let contents = message.as_bytes();
	try!(out.write_all(contents.len().to_string().as_bytes()));
	try!(out.write_all("\r\n\r\n".as_bytes()));
	try!(out.write_all(message.as_bytes()));
	try!(out.flush());
	Ok(())
}
