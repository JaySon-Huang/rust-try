use std::fs;
use std::path::Path;
use std::env;

fn main() {
    // TODO: list files under `./` if no param, else list files under param
    // Failed!
    // let path = match env::args().len() {
    //     1 => {
    //         let exec_path: &Path = Path::new(".");
    //         exec_path.parent().unwrap()
    //     }
    //     _ => {
    //         let arg = env::args().nth(1).unwrap();
    //         Path::new(&arg)
    //     }
    // };

    let arg = env::args().nth(1).unwrap();
    let path = match env::args().len() {
        1 => {
            let exec_path: &Path = Path::new(".");
            exec_path.parent().unwrap()
        }
        _ => {
            Path::new(&arg)
        }
    };

    // let arg = env::args().nth(0).unwrap();
    // let path = Path::new(&arg);

    // check if path is a dir
    let metadata = fs::metadata(path);
    println!("{:?} is dir? {:?}", path, metadata.is_dir());

    // for entry in try!(fs::read_dir(path)) {
    //     let dir = try!(entry);
    //     println!("{:?}", dir.path);
    // }
}
