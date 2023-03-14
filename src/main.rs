mod networkmods;
fn main()
{
    let mut Server = networkmods::tcp_server::TCPServer{};
    Server.init();
}