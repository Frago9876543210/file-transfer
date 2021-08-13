use argh::FromArgs;

use std::fs::File;
use std::io::{self, Write};
use std::net::{SocketAddr, TcpListener};
use std::path::PathBuf;

#[derive(FromArgs)]
#[argh(description = "Creates file-transfer server on given address")]
struct Args {
    #[argh(positional)]
    address: SocketAddr,
    #[argh(positional)]
    file: PathBuf,
}

fn main() -> io::Result<()> {
    let args = argh::from_env::<Args>();

    let listener = TcpListener::bind(args.address)?;

    let file = File::open(&args.file)?;
    let mmap = unsafe { memmap2::Mmap::map(&file) }?;

    let (mut stream, address) = listener.accept()?;

    println!("new client: {:?}", address);
    stream.write_all(&mmap)?;

    Ok(())
}
