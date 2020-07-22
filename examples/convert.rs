use bitcoincash_addr::{Address, AnyCodec, CashAddrCodec, Network};

fn main() {
    // Decode a base58 address
    let legacy_addr = "1NM2HFXin4cEQRBLjkNZAS98qLX9JKzjKn";
    let mut addr = Address::decode::<AnyCodec>(legacy_addr).unwrap();

    // Change the address to a testnet cashaddr
    addr.network = Network::Test;

    // Encode cashaddr
    let cashaddr_str = addr.encode::<CashAddrCodec>().unwrap();

    // bchtest:qr4zgpuznfg923ntyauyeh5v7333v72xhum2dsdgfh
    println!("{}", cashaddr_str);
}
