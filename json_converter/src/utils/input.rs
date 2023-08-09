use std::str::FromStr;

#[allow(unused)]
pub fn get_input<T: FromStr>() -> T {
    let mut line: String = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Get input error.");

    match line.trim().parse::<T>() {
        Ok(n) => n,
        Err(_) => panic!("Wrong input type err."),
    }
}
