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
    //let mut line_count = 0;

    // read through the lines 
    for line in lines {
        let mut parts = line.split(" ").collect::<Vec<&str>>();
        
        if is_line_safe(&parts) {
            //println!("line {}: is {}", line_count, is_safe);
            part1_safe += 1;
            part2_safe += 1;
        }
        else {
            // remove first element to cause line to be unsafe
            let pos = position_of_first_unsafe_element(&parts);
            parts.remove(pos);
        
            // then recheck if line safe, if now safe then add to part 2 safe
            if is_line_safe(&parts) {
                part2_safe += 1;
            }
        }
        //line_count += 1;
    }

    println!("part 1 safe: {}", part1_safe);
    println!("part 2 safe: {}", part2_safe);
}

fn is_line_safe(parts: &Vec<&str>) -> bool {
    let mut count = 0;
    let length = parts.len();
    //println!("length: {}", length);
    let mut direction = "";
    let mut is_safe = true;

    for part in parts {
       //println!("pos: {} - {}", count, *part); 
       if count + 1 >= length {
           //println!("End of Line!");
           continue;
       }
       let left = part.parse::<i32>().unwrap();
       let right = parts[count+1].parse::<i32>().unwrap();
       //println!("left: {} - right: {}", left, right);
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

// get the position of the first element that causes line to be unsafe
fn position_of_first_unsafe_element(parts: &Vec<&str>) -> usize {
    let mut pos = 0;
    //let mut pos_to_remove = 0;
    let length = parts.len();
    let mut direction = "";

    for part in parts {
        if pos + 1 >= length {
            continue;
        }

       let left = part.parse::<i32>().unwrap();
       let right = parts[pos+1].parse::<i32>().unwrap();
        
       let mut current_direction = "<";

       if left > right {
          current_direction = ">";
       }

       if direction == "" {
           direction = current_direction;
       }

       if direction != current_direction {
           //pos_to_remove = pos;
           break;
       }

       let distance = (left - right).abs();

       if distance < 1 || distance > 3 {
           //pos_to_remove = pos;
           break;
       }
       pos += 1;
    }

    return pos;
}
