use self::Server::{Connect,DisConnect,ServerError};
fn main() {
    let server = Server::Connect;
    match server{
        Connect => "Connected",
        _ => "No"
    };
    match server{
        Connect => "Connect",
        DisConnect | ServerError => "No care",
    };
}

enum Server {
    Connect,
    DisConnect,
    ServerError,
    //InternalError,
}
