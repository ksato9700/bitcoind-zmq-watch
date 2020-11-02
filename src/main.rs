use bitcoin::blockdata::block::Block;
use bitcoin::blockdata::transaction::Transaction;
use bitcoin::consensus::encode::deserialize;
use std::convert::TryInto;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let bitcoind_zmq_url = &args[1];

    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::SUB).unwrap();
    socket.set_subscribe(b"hashblock").unwrap();
    socket.set_subscribe(b"hashtx").unwrap();
    socket.set_subscribe(b"rawblock").unwrap();
    socket.set_subscribe(b"rawtx").unwrap();
    socket.set_subscribe(b"sequence").unwrap();
    socket
        .connect(&bitcoind_zmq_url)
        .expect("failed to connect");

    loop {
        let data = socket.recv_multipart(0).unwrap();
        let topic = String::from_utf8(data.get(0).unwrap().to_vec()).unwrap();

        let sequence_slice: Box<[u8]> = data.get(2).unwrap().to_vec().into_boxed_slice();
        let sequence_array: Box<[u8; 4]> = sequence_slice.try_into().unwrap();
        let sequence = u32::from_le_bytes(*sequence_array);
        println!("topic: {:?}", topic);
        println!("sequence: {:?}", sequence);

        let body = data.get(1).unwrap();
        if topic == "rawtx" {
            let tx: Transaction = deserialize(&body).unwrap();
            println!("tx: {:?}", tx);
        } else if topic == "rawblock" {
            let block: Block = deserialize(&body).unwrap();
            println!("block: {:?}", block);
        }
    }
}
