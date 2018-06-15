extern crate jsonrpc;
extern crate languageserver_types;
extern crate rust_lsp;
extern crate rustdt_util;
extern crate serde_json;

use rust_lsp::lsp_transport::*;
use rustdt_util::core::*;

#[test]
fn parse_transport_message_test() {
	use std::io::BufReader;

	let string = "Content-Length: 10 \r\n\r\n1234567890abcdef";
	assert_eq!(
		parse_transport_message(&mut BufReader::new(string.as_bytes())).unwrap(),
		"1234567890"
	);

	// Allow other header fields
	let string = "Content-Length: 13 \r\nContent-Blah\r\n\r\n1234\n567\r\n890abcdef";
	assert_eq!(
		parse_transport_message(&mut BufReader::new(string.as_bytes())).unwrap(),
		"1234\n567\r\n890"
	);

	// Test no-content
	let string = "\r\n\r\n1234567890abcdef";
	let err: GError = parse_transport_message(&mut BufReader::new(string.as_bytes())).unwrap_err();
	assert_eq!(&err.to_string(), "Content-Length: not defined or invalid.");

	// Test EOS
	let string = "";
	let err: GError = parse_transport_message(&mut BufReader::new(string.as_bytes())).unwrap_err();
	assert_eq!(&err.to_string(), "End of stream reached.");
}

#[test]
fn write_transport_message_test() {
	use rustdt_util::tests::*;

	let mut out: Vec<u8> = vec!['x' as u8];
	write_transport_message(&"1234\n67", &mut out).unwrap();

	assert_equal(
		String::from_utf8(out).unwrap(),
		"xContent-Length: 7\r\n\r\n1234\n67".to_string(),
	);
}
