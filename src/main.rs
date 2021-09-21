use std::env;

fn main() {
    if env::args().count() < 2 {
        eprintln!("path not specified");
        return;
    }

    let path = env::args().into_iter().last().expect("path not specified");

    println!("openning {}", &path);
    let connection = sqlite::open(path).unwrap();

    connection.execute("vacuum;").unwrap();
    println!("success");
}
