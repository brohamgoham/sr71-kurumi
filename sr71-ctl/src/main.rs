use clap::Parser;
use sr71_core::emote;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    emote: Option<String>
}

fn main() {
    let args = Args::parse();

    match args.emote.as_deref() {
        Some("coding") => println!("CLI: set coding emote"),
        Some("happy") => println!("CLI: set happy emote"),
        Some("sleepy") => println!("CLI: set sleepy emote"),
        Some("annoyed") => println!("CLI: set annoyed emote"),
        Some("gitsuccess") => println!("CLI: set Git success emote"),
        Some("gifail") => println!("CLI: set Git fail emote"),
        Some("studying") => println!("CLI: set studying emote"),
        _ => println!("CLI: Command not found")

    }

}
