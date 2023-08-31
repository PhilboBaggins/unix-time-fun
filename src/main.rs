use unix_ts_macros::ts;

fn print_time(n: i64) {
    let t = ts!(n);
    println!("{:#08X} {}", n, t.to_utc_datetime());
}

fn main() {
    /*
    print_time(0x00000000);
    print_time(0x000F0000);
    print_time(0x00100000);
    print_time(0x00FF0000);
    print_time(0x01000000);
    print_time(0x0FFF0000);

    print_time(0x10000000);
    print_time(0x20000000);
    print_time(0x30000000);
    print_time(0x40000000);
    print_time(0x50000000);
    print_time(0x60000000);
    print_time(0x70000000);
    print_time(0x80000000);
    print_time(0x90000000);
    print_time(0xA0000000);
    print_time(0xB0000000);
    print_time(0xC0000000);
    print_time(0xD0000000);
    print_time(0xE0000000);
    print_time(0xF0000000);
    */

    print_time(0x60606060);
    print_time(0x61616161);
    print_time(0x62626262);
    print_time(0x63636363);
    println!("");
    
    print_time(0x60000000);
    print_time(0x66000000);
    print_time(0x66600000);
    print_time(0x66660000);
    print_time(0x66666666);
}
