use std::net::TcpListener;
use anyhow::Result;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    /*
        Iterating over connection attempts. Many operating systems have a limit to the number of
        simultaneous open connections they can support; new connection attempts beyond that number
        will produce an error until some of the open connections are closed.
     */
    for stream in listener.incoming() {
        println!("Connection established: {:?}", stream?);
    }
    /*
        When stream goes out of scope and is dropped at the end of the loop,
        the connection is closed as part of the drop implementation.
     */

    Ok(())
}
