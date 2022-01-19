fn main(){
    let penguin_data = "\
    common name, length (cm)
    Little penguin, 33
    Yello-eyed penguin, 65
    Fiordlang penguin, 60
    Invalid, data
    ";

    let records = penguin_data.lines();
    for (index, record) in records.enumerate() {
        if index == 0 || record.trim().is_empty() {
            continue;
        }

         // <_>表示Vec中的元素类型由编译器自行推断，在很多场景下，都会帮我们省却不少功夫
        let fields: Vec<_> = record.split(',').map(|f|f.trim()).collect();
        if cfg!(debug_assertions) {
            eprintln!("debug: {record} -> {:?}", fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{name}, {length}cm");
        }
    }
}