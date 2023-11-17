extern crate proc_macro;
// example of a main that does basic File I/O
// fn main() {
//     // TODO: ensure that this script is run EVERYTIME a Schema object changes
//     let path = Path::new("src/hello_world.txt");
//     let display = path.display();
//     // Open the path in read-only mode, returns `io::Result<File>`
//     let mut file = match File::open(&path) {
//         Err(why) => panic!("couldn't open {}: {}", display, why),
//         Ok(file) => file,
//     };
//
//     // Read the file contents into a string, returns `io::Result<usize>`
//     let mut s = String::new();
//     match file.read_to_string(&mut s) {
//         Err(why) => panic!("couldn't read {}: {}", display, why),
//         Ok(_) => print!("{} contains:\n{}", display, s),
//     }
// }
//

fn main() {}
