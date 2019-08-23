use std::io;
use std::io::Write;
use std::u64;

const BIT_TABLE: [u8; 16] = [0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4];

fn get_parity_bit(data: u64) -> u8 {
    let mut odd_bit_count: u8 = 0;
    let mut step: u8 = 0;

    let mut n: u64 = data;
    let mut index: u64;

    for _i in 0..16 {
        n = n >> step;

        index = n & 0x00000000_0000000F;
        odd_bit_count += BIT_TABLE[index as usize];

        step = 4;
    }

    if odd_bit_count % 2 == 0 {
        return 0;
    }

    return 1;
}

fn get_parity(data: u64) -> u8 {
    let p1 = get_parity_bit(data & 0xDAB5556A_AAAAAAD5);
    let p2 = get_parity_bit(data & 0xB66CCCD9_999999B3);
    let p3 = get_parity_bit(data & 0x71E3C3C7_8787878F);
    let p4 = get_parity_bit(data & 0x0FE03FC0_7F807F80);
    let p5 = get_parity_bit(data & 0x001FFFC0_007FFF80);
    let p6 = get_parity_bit(data & 0x0000003F_FFFFFF80);
    let p7 = get_parity_bit(data & 0x00000000_0000007F);

    // p8 doesn't include p1 ~ p7.
    //let p8 = get_parity_bit(data & 0xFFFFFFFF_FFFFFFFF);

    // p8 includes p1 ~ p7.
    let p8 = get_parity_bit(data & 0xED3A65B4_CB4B34E9);

    println!("p1 = {}", p1);
    println!("p2 = {}", p2);
    println!("p3 = {}", p3);
    println!("p4 = {}", p4);
    println!("p5 = {}", p5);
    println!("p6 = {}", p6);
    println!("p7 = {}", p7);
    println!("p8 = {}", p8);

    // The MSB is p8
    let parity_bit = p8 << 7 | p7 << 6 | p6 << 5 | p5 << 4 | p4 << 3 | p3 << 2 | p2 << 1 | p1;

    return parity_bit;
}

fn main() {
    let mut num: String;

    let mut data: u64;
    let mut p: u8;

    loop {
        print!("Please input hex integer(8 byte - 8F7F6F5F4F3F2F1F): ");
        io::stdout().flush().expect("Can't flush buffer");

        num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");

        num = num.trim().to_string();

        if num == "q" {
            println!("Bye Bye");
            break;
        }

        if num.len() != 16 {
            println!("Please input hex integer with 16 {} {}", num.len(), num);
            continue;
        }

        println!("");

        data = u64::from_str_radix(&num, 16).expect("Failed to convert to integer");
        p = get_parity(data);

        println!("--------------------------------------");
        println!("data    = 0x{:016X}", data);
        println!("parity  = 0x{:02X}", p);
        println!("!parity = 0x{:02X}", !p);
        println!("-------------------------------------- \n");
    }
}
