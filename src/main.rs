use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration
};

type Result = anyhow::Result<()>;

fn main() -> Result {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    /*
        Iterating over connection attempts. Many operating systems have a limit to the number of
        simultaneous open connections they can support; new connection attempts beyond that number
        will produce an error until some of the open connections are closed.
     */
    for stream in listener.incoming() {
        handle_connection(stream?)?;
    }

    Ok(())

    /*
        When stream goes out of scope and is dropped at the end of the loop,
        the connection is closed as part of the drop implementation.
     */
}

fn handle_connection(mut stream: TcpStream) -> Result {
    let reader = BufReader::new(&mut stream);
    // first line is always of the form: "GET / HTTP/1.1"
    let request_line = reader.lines().next().unwrap()?;

    let (status, file) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(file)?;
    let length = contents.len(); // ensures a valid HTTP response
    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())?;

    Ok(())

    /*
    let http_request: Vec<_> = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    */
}
