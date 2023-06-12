extern crate inquire;

fn main() {
    let mut path_to_discord_src = inquire::Text::new("Path to discord src: ")
        .prompt()
        .unwrap();

    // Add a / to the end of the path, if it doesn't already exist
    if path_to_discord_src.chars().collect::<Vec<char>>()[path_to_discord_src.len() - 1] != '/' {
        path_to_discord_src = path_to_discord_src + "/";
    }

    let path_to_file = path_to_discord_src + "9c02eda6c6d3193f8589.js";

    let string = std::fs::read_to_string(path_to_file).unwrap();

    let start_pos = string.find("buildNumber:").unwrap() + "buildNumber:".len() + 1;

    let mut build_number_str = String::new();
    let string_chars = string.as_bytes();
    for i in 0..6 {
        build_number_str += char::from(string_chars[start_pos + i]).to_string().as_str();
    }

    println!("{}", build_number_str);
}
