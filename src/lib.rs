mod quotes;

#[doc = "returns a generated Korwin quote"]
pub fn generate() -> String {
    quotes::get()
        .into_iter()
        .map(|s| {
            let num = rand::random::<usize>() % s.len();
            s[num].to_string()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[doc = "returns the amount of possible results"]
pub fn variations() -> usize {
    quotes::get().into_iter().map(|s| s.len()).product()
}
