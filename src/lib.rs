/*!

### RustLSP

# Examples:

See full server/client example here:
https://github.com/RustDT/RustLSP/blob/master/src/server_tests.rs

*/

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate serde;
extern crate serde_json;

pub extern crate jsonrpc;
pub extern crate languageserver_types as ls_types;
pub extern crate rustdt_util as util;

#[macro_use]
extern crate log;

pub mod lsp;
pub mod lsp_transport;
