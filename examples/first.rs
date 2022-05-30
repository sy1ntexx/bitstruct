bitstruct::bitstruct! {
    #[derive(Default)]
    struct Va : u64 {
        page: 0..=11,
        pt: 12..=20,
        pd: 21..=29,
        pdpt: 30..=38,
        pml4: 39..=47
    }
}

fn main() {
    let t = Va::from_bits(0x7ff60bf40190);
    println!("{:x}", t.page());
    println!("{:x}", t.pt());
    println!("{:x}", t.pd());
    println!("{:x}", t.pdpt());
    println!("{:x}", t.pml4());
}
