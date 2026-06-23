use clap::Parser;
use std::env;
use urlencoding::encode;
use reqwest::blocking::get;

const DEFAULT_MESSAGE: &str = "This is the default message from neu-send-signal";

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Message text is optional but recommended, as the default is probably not what you want.
    #[arg(long)]
    text: Option<String>,
}

fn main() {
    let args = Args::parse();

    let phone = env::var("CALLMEBOT_MYPHONE").expect("CALLMEBOT_MYPHONE must be set");
    let apikey = env::var("CALLMEBOT_APIKEY").expect("CALLMEBOT_APIKEY must be set");
    let message = args.text.unwrap_or_else(|| DEFAULT_MESSAGE.to_string());
    let encoded = encode(&message);

    let url = format!(
        "https://signal.callmebot.com/signal/send.php?phone={}&apikey={}&text={}",
        phone, apikey, encoded
    );

    // println!("→ Sending Signal message via neu-send-signal");
    // println!("→ Message: {}", message);
    
    match get(&url) {
        Ok(_resp) => {
                println!("✓ Message Sent\n");
            }
        Err(request_error) => eprintln!("✗ Request failed: {}", request_error),
    }
}

