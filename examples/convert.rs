use bitcoincash_addr::{Address, Base58Codec, Network};

fn main() {
    // Decode a base58 address
    let legacy_addr = "1NM2HFXin4cEQRBLjkNZAS98qLX9JKzjKn";
    let mut base58 = Address::<Base58Codec>::decode(legacy_addr).unwrap();

    // Change the address to a test net cashaddr
    base58.network = Network::Test;
    let cashaddr = base58.into_cashaddr();

    // Encode cashaddr
    let cashaddr_str = cashaddr.encode().unwrap();

    // bchtest:qr4zgpuznfg923ntyauyeh5v7333v72xhum2dsdgfh
    println!("{}", cashaddr_str);
}
