extern crate inquire;

fn main() {
    let mut path_to_discord_src = inquire::Text::new("Path to discord src: ")
        .prompt()
        .unwrap();

    // Add a / to the end of the path, if it doesn't already exist
    if path_to_discord_src.chars().collect::<Vec<char>>()[path_to_discord_src.len() - 1] != '/' {
        path_to_discord_src = path_to_discord_src + "/";
    }

    let files_in_dir = std::fs::read_dir(path_to_discord_src).unwrap();

    for file in files_in_dir {
        let string = std::fs::read_to_string(file.unwrap().path()).unwrap();

        let find_res = string.find("buildNumber:");

        if find_res.is_none() {
            continue;
        }

        let start_pos = find_res.unwrap() + "buildNumber:".len() + 1;

        let mut build_number_str = String::new();
        let string_chars = string.as_bytes();
        for i in 0..6 {
            build_number_str += char::from(string_chars[start_pos + i]).to_string().as_str();
        }

        println!("{}", build_number_str);
    }
}
