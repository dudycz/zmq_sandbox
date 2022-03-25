const DATA: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor \
incididunt ut labore et dolore magna aliqua. Orci ac auctor augue mauris augue neque gravida in. \
Et ligula ullamcorper malesuada proin libero nunc. Tellus integer feugiat scelerisque varius morbi \
enim nunc. Arcu vitae elementum curabitur vitae. Feugiat nisl pretium fusce id. Id velit ut tortor \
pretium. Magna ac placerat vestibulum lectus. Fermentum et sollicitudin ac orci phasellus egestas \
tellus rutrum. Lectus proin nibh nisl condimentum id venenatis a condimentum. Egestas dui id \
ornare arcu odio ut sem nulla. Id diam vel quam elementum. Purus non enim praesent elementum \
facilisis leo vel fringilla est. Amet dictum sit amet justo donec enim. Risus in hendrerit gravida \
rutrum quisque non. Orci ac auctor augue mauris augue neque gravida in. Eget nulla facilisi etiam \
dignissim diam quis enim lobortis scelerisque. Amet justo donec enim diam vulputate ut. Molestie \
nunc non blandit massa enim nec dui. Velit euismod in pellentesque massa. Lorem dolor sed viverra \
ipsum nunc aliquet bibendum enim. Est placerat in egestas erat imperdiet sed euismod. Vulputate \
enim nulla aliquet porttitor lacus luctus. Nec ultrices dui sapien eget mi proin sed libero enim. \
Pharetra convallis posuere morbi leo. Arcu dui vivamus arcu felis bibendum ut tristique. Praesent \
semper feugiat nibh sed pulvinar proin. In nibh mauris cursus mattis. Nullam eget felis eget nunc \
lobortis mattis aliquam faucibus purus. Dui sapien eget mi proin. Dolor morbi non arcu risus quis \
varius quam quisque. Donec adipiscing tristique risus nec. Nunc congue nisi vitae suscipit tellus \
mauris a diam maecenas. Diam in arcu cursus euismod quis viverra nibh cras pulvinar. Pharetra diam \
sit amet nisl suscipit adipiscing bibendum est ultricies. Nullam non nisi est sit amet facilisis \
magna etiam tempor. Volutpat est velit egestas dui id ornare arcu. Sit amet facilisis magna etiam \
tempor orci eu lobortis. Elit sed vulputate mi sit. At quis risus sed.";

const IP_ADDRESS: &str = "tcp://127.0.0.1";
const SERVER_PORT: u16 = 12345;
const CLIENT_PORT: u16 = 12346;
use std::{thread, time};
use std::time::Duration;

fn create_socket_and_connect(port: u16, sock_type: zmq::SocketType, ctx: &zmq::Context) -> zmq::Socket {
    let socket = ctx.socket(sock_type).unwrap();
    let addr = format!("{}:{}", IP_ADDRESS, port);
    socket.connect(addr.as_str()).unwrap();
    socket
}

fn main() {
    let ctx = zmq::Context::new();
    let rx = create_socket_and_connect(CLIENT_PORT, zmq::PULL, &ctx);
    let tx = create_socket_and_connect(SERVER_PORT, zmq::PUSH, &ctx);

    //let ten_nanos = Duration::from_nanos(5);
    for i in 1..10_000_000 {
        tx.send(DATA, 0);
        //thread::sleep(ten_nanos);
    }
    tx.send("BYE", 0);
    let mut msg = zmq::Message::new();
    rx.recv(&mut msg,0);
}
