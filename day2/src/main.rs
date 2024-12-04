use rand::Rng;

fn generate_tree(size: i32, ornaments_count: i32) {
    let mut rng = rand::thread_rng();

    for height in 0..=size {
        let indent = size - height;

        // Define the pattern for the tree branches
        let pattern: String;
        if height == 0 {
            pattern = "*".to_string();  // Star at the top
        } else {
            if height % 2 == 0 {
                pattern = "/_\\".to_string();  // Pattern for even rows
            } else {
                pattern = "/_/".to_string();  // Pattern for odd rows
            }
        }

        let mut line = " ".repeat(indent as usize);

        if height == 0 {
            line.push_str(&pattern);
        } else {
            // Repeat the pattern based on the height
            for _ in 0..height {
                line.push_str(&pattern);
            }

            // Convert line into a mutable vector of characters to replace underscores with ornaments
            let mut line_chars: Vec<char> = line.chars().collect();
            let mut ornaments_added = 0;

            // Add ornaments randomly in positions where underscores (_) exist
            for i in 1..line_chars.len() - 1 {
                if ornaments_added < ornaments_count && line_chars[i] == '_' && rng.gen_bool(0.15) {
                    // Replace underscore with a Christmas ornament (○)
                    line_chars[i] = '○';
                    ornaments_added += 1;
                }
            }

            // Ensure the last character is a backslash
            if let Some(last_char) = line_chars.last_mut() {
                *last_char = '\\';
            }

            line = line_chars.into_iter().collect();
        }

        // Limit the length of the line to avoid excess characters
        let max_length = size * 2;
        let line = line.chars().take(max_length as usize).collect::<String>();

        // Print the tree line
        println!("{}", line);

        // Print the trunk after the last row
        if height == size {
            print!("{}", " ".repeat((size - 2) as usize));
            println!("[___]");
        }
    }
}

fn main() {
    let height_tree = 20;
    let ornaments_count = 10;
    generate_tree(height_tree, ornaments_count);
}




    //           \ /
    //         -->*<--
        //       /_\
        //      /_\_\
        //     /_/_/_\
        //    /_\_\_\_\
        //   /_/_/_/_/_\
    //      /_\_\_\_\_\_\
    //     /_/_/_/_/_/_/_\
    //    /_\_\_\_\_\_\_\_\
    //   /_/_/_/_/_/_/_/_/_\
    //  /_\_\_\_\_\_\_\_\_\_\
    // /_/_/_/_/_/_/_/_/_/_/_\
    //          [___]