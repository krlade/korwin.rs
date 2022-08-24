mod korwin;

fn main() {
    let output = generate();
    println!("{output}");
}

fn generate() -> String {
    let korwin = korwin::get()
        .clone()
        .into_iter()
        .map(|s| {
            let num = rand::random::<usize>() % s.len();
            let mut string = s[num].to_string();
            string.push(' ');
            string
        })
        .collect::<String>();
    korwin
}
