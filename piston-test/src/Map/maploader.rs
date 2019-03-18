pub mod Map
{
    use std::fs::File;
    use std::io::prelude::*;

    pub struct Map {
        tile : Vec<Vec<u64>>,
        size : (i32, i32)
    }

    fn load_file(path : String) -> Option<String> {

        let file_path = File::open(path);
        let mut file : File;
        let mut map_string = String::new();

        match file_path {
            Ok(res) => {file = res},
            Err(_) => return None
        }

        return match file.read_to_string(&mut map_string) {
            Ok(_) => {Some(map_string)},
            Err(_) => None
        }
    }
}