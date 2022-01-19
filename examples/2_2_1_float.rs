fn main(){
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let zyx: (f64, f64, f64) = (0.1, 0.2, 0.3);
    println!("abc (f32)");
    println!("    0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("          0.3: {:x}", abc.2.to_bits());
    println!();

    println!("xyz (f64)");
    println!("    0.1 + 0.2: {:x}", (zyx.0 + zyx.1).to_bits());
    println!("          0.3: {:x}", zyx.2.to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    assert!(zyx.0 + zyx.1 == zyx.2);
}