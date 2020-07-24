fn stringify(val: &str) -> String {
   "booh".to_owned()
}

include!(concat!(env!("OUT_DIR"), "/canonicaljson.uniffi.rs"));