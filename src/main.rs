use anyhow::Context;
use uuid::Uuid;

fn main() {
    let f = "foo".parse::<Uuid>().context("parsing").unwrap_err();
    eprintln!("top-level (anyhow) error:");
    eprintln!("     formatted: {}", f);
    eprintln!("    alt format: {:#}", f);
    eprintln!("anyhow debug format:");
    eprintln!("{:?}\n", f);
    eprintln!("CAUSE CHAIN");
    for (i, error) in f.chain().enumerate() {
        eprintln!("chain {}: {:?}", i, error);
        eprintln!("     formatted: {}", error);
        eprintln!("    alt format: {:#}", error);
        eprintln!("\n");
    }
}
