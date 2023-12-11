use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn process_line(line: &str) -> HashMap<String, i32> {
    // Split the line into segments
    let segments = line.split(';');

    let mut scores = HashMap::new();

    // Process each segment
    for segment in segments {
        // Split the segment into individual color counts
        let color_counts = segment.split(',');

        for color_count in color_counts {
            // Split each color count into the number and the color
            let parts: Vec<&str> = color_count.trim().split_whitespace().collect();
            if parts.len() == 2 {
                let count: i32 = parts[0].parse().unwrap_or(0);
                let color = parts[1];
                // match parts[1] {
                //     "blue" => blue_count += count,
                //     "red" => red_count += count,
                //     "green" => green_count += count,
                //     _ => {},
                // }
                scores
                    .entry(String::from(color))
                    .and_modify(|e| *e = std::cmp::max(*e, count))
                    .or_insert(count);
            }
        }
    }

    // Print the summarized counts
    println!("{}", line);
    for (color, count) in &scores {
        println!("Total {}: {}", color, count);
    }

    scores
}

fn main() {
    // Define the path to the file
    let path = Path::new("input.txt");

    // Open the file in read-only mode (ignoring errors)
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    // Create a new Buffered Reader and iterate over lines
    let reader = io::BufReader::new(file);
    let mut sum = 0;
    let mut sum_part_2 = 0;
    let mut counter = 1;
    for line in reader.lines() {
        let line = line.expect("Error reading line!");

        let count_map = process_line(&line);
        if *count_map.get("red").unwrap_or(&0) <= 12
            && *count_map.get("green").unwrap_or(&0) <= 13
            && *count_map.get("blue").unwrap_or(&0) <= 14
        {
            sum += counter;
            println!("current sum is {}", sum);
        }

        sum_part_2 += *count_map.get("red").unwrap_or(&0) * *count_map.get("green").unwrap_or(&0) * *count_map.get("blue").unwrap_or(&0);
        println!("current sum for part 2 is {}", sum_part_2);
        counter += 1;
    }

    println!("sum for part1 is {}", sum);
    println!("sum for part2 is {}", sum_part_2);

}
