use std::fs;


pub fn get_files_from_dir(rtn_full_path: bool, sort_filetype: &str, path: &str) -> Vec<String>{
    let files = match fs::read_dir(path) {
        Ok(s) => s,
        Err(..) => {
            // println!("DIRECTORY is not found ERROR");
            // println!("{:?}", path);
            // match create_dir(path) {
            //     Ok(s) => {}
            //     Err(err) => create_dir_all("data/".to_owned() + path).unwrap()
            // }
            return vec![]
        }
    };
    if rtn_full_path {
        let mut rtn = Vec::new();
        for entry in files {
            let dir = entry.unwrap();
            rtn.push(String::from(dir.path().to_str().unwrap()));
            // println!("{:?}", dir.path());
        }
        rtn
    }
    else {
        let name = files.filter_map(|entry| {
            entry.ok().and_then(|e|
                e.path().file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            )
        }).collect::<Vec<String>>();
        if sort_filetype == "" {
            name
        }else {
            let mut sorted_name = Vec::new();
            for i in name {
                let v: String = i.chars().rev().take(sort_filetype.len()).collect();
                let f:String = sort_filetype.chars().rev().collect();
                if v == f {
                    sorted_name.push(i);
                }
            }
            sorted_name
        }
    }
}


