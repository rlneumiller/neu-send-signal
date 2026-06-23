use clap::Parser;
use std::env;
use urlencoding::encode;
use reqwest::blocking::get;

const DEFAULT_MESSAGE: &str = "This is the default message from neu-send-signal";

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Send a Signal message via CallMeBot.",
    long_about = "A tiny Rust CLI that sends a Signal message through CallMeBot.\n\
                  If no message text is provided, a default message is used.\n\
                  \n\
                  REQUIRED ENVIRONMENT VARIABLES:\n\
                    • CALLMEBOT_MYPHONE  – Your CallMeBot phone ID (UUID-like string)\n\
                    • CALLMEBOT_APIKEY   – Your CallMeBot API key\n\
                  \n\
                  Example:\n\
                    export CALLMEBOT_MYPHONE=\"12345678-1234-1234-1234-1234567890ab\"\n\
                    export CALLMEBOT_APIKEY=\"123456\"\n\
                  "
)]
struct Args {
    /// Message text. If omitted, a default message is used.
    #[arg(value_name = "MESSAGE")]
    message: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let phone = env::var("CALLMEBOT_MYPHONE").expect("CALLMEBOT_MYPHONE must be set");
    let apikey = env::var("CALLMEBOT_APIKEY").expect("CALLMEBOT_APIKEY must be set");

    let message = if args.message.is_empty() {
        DEFAULT_MESSAGE.to_string()
    } else {
        args.message.join(" ")
    };

    let encoded = encode(&message);

    let url = format!(
        "https://signal.callmebot.com/signal/send.php?phone={}&apikey={}&text={}",
        phone, apikey, encoded
    );

    // println!("→ Message: {}", message);
    
    match get(&url) {
        Ok(_) => println!("✓ Message Sent\n"),
        Err(e) => eprintln!("✗ Request failed: {}", e),
    }
}
