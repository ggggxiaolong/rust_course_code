fn main() {
    let tweenty = 20;
    let tweenty_one: i32 = 21;
    let tweenty_two = 22_i32;
    let addition = tweenty + tweenty_one + tweenty_two;
    println!("{tweenty} + {tweenty_one} + {tweenty_two} = {addition}");

    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2));

    let forty_two = [
        42.0,
        42f32,
        42.0_f32,
    ];
    println!("{:02}", forty_two[0]);
}
