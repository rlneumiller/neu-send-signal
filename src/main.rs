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
    /// Message text. Must be a single quoted string unless --shell-safe or --raw is used.
    #[arg(value_name = "MESSAGE")]
    message: Vec<String>,

    /// Allow unquoted multi-word messages (auto-joined).
    #[arg(long)]
    shell_safe: bool,

    /// Accept message exactly as provided, with no validation or joining rules.
    #[arg(long)]
    raw: bool,
}


fn main() {
    let args = Args::parse();

    // Enforce quoting unless --shell-safe or --raw is used
    if !args.shell_safe && !args.raw && args.message.len() > 1 {
        eprintln!("Error: Message text must be provided as a single quoted string.");
        eprintln!("Hint: Use --shell-safe for auto-joining or --raw for literal input.");
        eprintln!("Example: neu-send-signal \"Hello from neu-send-signal\"");
        std::process::exit(1);
    }

    let phone = env::var("CALLMEBOT_MYPHONE")
        .expect("CALLMEBOT_MYPHONE must be set");
    let apikey = env::var("CALLMEBOT_APIKEY")
        .expect("CALLMEBOT_APIKEY must be set");

    let message = if args.message.is_empty() {
        DEFAULT_MESSAGE.to_string()
    } else if args.raw {
        // Raw mode: take arguments exactly as provided, joined with spaces
        args.message.join(" ")
    } else if args.shell_safe {
        // Shell-safe mode: join safely
        args.message.join(" ")
    } else {
        // Strict mode: must be a single quoted string
        args.message[0].clone()
    };

    let encoded = encode(&message);

    let url = format!(
        "https://signal.callmebot.com/signal/send.php?phone={}&apikey={}&text={}",
        phone, apikey, encoded
    );

    match get(&url) {
        Ok(_) => println!("✓ Message Sent\n"),
        Err(e) => eprintln!("✗ Request failed: {}", e),
    }
}
