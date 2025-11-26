use std::env;
use std::fs;
use std::path::Path;

// P11CHANGE-PACKET-SIZE-INCREASE
fn main() {
    // Default packet size (IPv6 minimum MTU - headers)
    let default_packet_size = 1280 - 40 - 8; // 1232
    
    // Read packet size from environment variable, fallback to default
    let packet_size = env::var("SOLANA_PACKET_DATA_SIZE")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(default_packet_size);
    
    // Generate the constant
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("packet_size.rs");
    
    let contents = format!(
        "/// Maximum over-the-wire size of a Transaction\n\
         /// Set via SOLANA_PACKET_DATA_SIZE environment variable (default: {})\n\
         pub const PACKET_DATA_SIZE: usize = {};\n",
        default_packet_size,
        packet_size
    );
    
    fs::write(&dest_path, contents).unwrap();
    
    // Tell cargo to rerun this build script if the environment variable changes
    println!("cargo:rerun-if-env-changed=SOLANA_PACKET_DATA_SIZE");
    
    // Also rerun if this build script changes
    println!("cargo:rerun-if-changed=build.rs");
}