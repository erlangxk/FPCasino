extern crate jsonrpc_core;
#[macro_use]
extern crate jsonrpc_macros;
extern crate jsonrpc_http_server;
//use jsonrpc_http_server::{ServerBuilder, DomainsValidation, AccessControlAllowOrigin, RestApi};
use jsonrpc_core::Result;

use std::collections::HashMap;

build_rpc_trait! {
	pub trait Rpc {
		/// Adds two numbers and returns a result
		#[rpc(name = "add")]
		fn add(&self, u64, u64) -> Result<u64>;

        #[rpc(name = "xxx")]
		fn xxx(&self, u64, HashMap<String,u64>) -> Result<u64>;
	}
}

pub struct RpcImpl;
impl Rpc for RpcImpl {
	fn add(&self, a: u64, b: u64) -> Result<u64> {
		Ok(a + b)
	}

    fn xxx(&self,a:u64, b:HashMap<String,u64>)->Result<u64>{
        Ok(a+b.get("a").unwrap()+b.get("b").unwrap())
    }
}


fn main() {
	let mut io = jsonrpc_core::IoHandler::new();
	io.extend_with(RpcImpl.to_delegate());

    

    let request = r#"{"jsonrpc": "2.0", "method": "xxx", "params": [42, {"a":300, "b":4000}], "id": 1}"#;
    let r = io.handle_request_sync(request);
	println!("{:?}", r);
}