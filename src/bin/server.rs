const IP_ADDRESS: &str = "tcp://127.0.0.1";
const SERVER_PORT: u16 = 12345;
const CLIENT_PORT: u16 = 12346;

fn create_socket_and_bind(port: u16, sock_type: zmq::SocketType, ctx: &zmq::Context) -> zmq::Socket {
    let socket = ctx.socket(sock_type).unwrap();
    let addr = format!("{}:{}", IP_ADDRESS, port);
    socket.bind(addr.as_str()).unwrap();
    socket
}

fn main() {
    let ctx = zmq::Context::new();
    let rx = create_socket_and_bind(SERVER_PORT, zmq::PULL, &ctx);
    let tx = create_socket_and_bind(CLIENT_PORT, zmq::PUSH, &ctx);

    let mut dummy_sockets = Vec::new();
    for i in 10..20 {
        dummy_sockets.push(create_socket_and_bind(SERVER_PORT+i, zmq::PULL, &ctx));
    }

    let mut items = [
        rx.as_poll_item(zmq::POLLIN),
        dummy_sockets[0].as_poll_item(zmq::POLLIN),
        dummy_sockets[1].as_poll_item(zmq::POLLIN),
        dummy_sockets[2].as_poll_item(zmq::POLLIN),
        dummy_sockets[3].as_poll_item(zmq::POLLIN),
        dummy_sockets[4].as_poll_item(zmq::POLLIN),
    ];

    let mut msg = zmq::Message::new();
    let mut finish = false;
    while !finish {
        // // Poll with drain
        zmq::poll(&mut items, -1).unwrap();
        if items[0].is_readable() {
            while rx.recv(&mut msg, zmq::DONTWAIT).is_ok() {
                if msg.as_ref() == "BYE".as_bytes() {
                    tx.send("BYE", 0).unwrap();
                    finish = true;
                }
            }
        }

        // Poll with single recv()
        // zmq::poll(&mut items, -1).unwrap();
        // if items[0].is_readable() {
        //     if rx.recv(&mut msg, zmq::DONTWAIT).is_ok() {
        //         if msg.as_ref() == "BYE".as_bytes() {
        //             tx.send("BYE", 0).unwrap();
        //             finish = true;
        //         }
        //     }
        // }

        // recv() version
        // if rx.recv(&mut msg, 0).is_ok() {
        //     if msg.as_ref() == "BYE".as_bytes() {
        //         tx.send("BYE",0).unwrap();
        //         finish = true;;
        //     }
        // }
    }
}
