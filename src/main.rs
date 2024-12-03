use std::env;
use std::fs;

fn main() {
    // get the command line arguments
    let args: Vec<String> = env::args().collect();
    
    // check for filename
    if args.len() < 2 {
        println!("No file name specified!");
        return;
    }

    // get filename from the first argument
    let file_path = &args[1];
    let buffer = fs::read_to_string(file_path).unwrap();
    let lines = buffer.lines();
    let mut part1_safe = 0;
    let mut part2_safe = 0;
    let mut line_count = 0;

    // read through the lines 
    for line in lines {
        let parts = line.split(" ").collect::<Vec<&str>>();
        
        if is_line_safe(&parts) {
            println!("✅ line {}: {}", line_count, line);
            part1_safe += 1;
            part2_safe += 1;
        }
        else {
            println!("❌ line {}: {}", line_count, line);
           
            //try each version of the line with one element removed and see if it allows the line
            //to pass inspection
            let length = parts.len();
            for pos in 0..length {
                let mut shortned: Vec<&str> = Vec::new();
                for(i, el) in parts.iter().enumerate() {
                    if i != pos {
                        shortned.push(el);
                    }
                }
                let short = shortned.join(" ");
                if is_line_safe(&shortned) {
                    part2_safe += 1;
                    println!("✅ line {}: {}", line_count, short);
                    break;
                }
                else {
                    println!("❌ line {}: {}", line_count, short);
                } 
            }
        }
        line_count += 1;
    }

    println!("part 1 safe: {}", part1_safe);
    println!("part 2 safe: {}", part2_safe);
}

fn is_line_safe(parts: &Vec<&str>) -> bool {
    let mut count = 0;
    let length = parts.len();
    let mut direction = "";
    let mut is_safe = true;

    for part in parts {
       if count + 1 >= length {
           continue;
       }
       let left = part.parse::<i32>().unwrap();
       let right = parts[count+1].parse::<i32>().unwrap();
       let mut current_direction = "<";

       if left > right {
          current_direction = ">";
       }

       if direction == "" {
           direction = current_direction;
       }

       if direction != current_direction {
           is_safe = false;
           break;
       }

       let distance = (left - right).abs();

       if distance < 1 || distance > 3 {
          is_safe = false;
          break;
       }

       count += 1;
    }
    return is_safe;
}

