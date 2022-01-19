fn main (){
    let x = 'a';
    println!("字符占用了{}字节的内存大小", std::mem::size_of_val(&x));
}