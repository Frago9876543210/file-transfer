use argh::FromArgs;

use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::PathBuf;

#[derive(FromArgs)]
#[argh(description = "Connects to file-transfer server with given address")]
struct Args {
    #[argh(positional)]
    address: SocketAddr,
    #[argh(positional)]
    file: PathBuf,
}

fn main() -> io::Result<()> {
    let args = argh::from_env::<Args>();

    let mut stream = TcpStream::connect(args.address)?;

    let mut buf = Vec::new();
    stream.read_to_end(&mut buf)?;

    let mut file = File::create(args.file)?;
    file.write_all(&buf)?;

    Ok(())
}
