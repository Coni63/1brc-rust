use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn convert_str_to_i32(s: &str) -> i32 {
    // as fast as s.replace(".", "")
    let n = s.len();
    let number: String = format!("{}{}", &s[0..n - 2], &s[n - 1..]);
    number.parse::<i32>().ok().unwrap()
}

pub fn run_v4() {
    let mut cities: HashMap<String, usize> = HashMap::new();
    let mut values: [[i32; 4]; 10000] = [[0, 0, 0, 0]; 10000];
    let mut num_inserted = 0;

    let path = Path::new("measurements-10000000.txt");

    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(content) => {
                    let parts: Vec<&str> = content.splitn(2, ';').collect();
                    let name = parts[0];
                    let temp = convert_str_to_i32(parts[1]);

                    if !cities.contains_key(name) {
                        cities.insert(name.to_string(), num_inserted);
                        num_inserted += 1;
                    }

                    let &id = cities.get(name).unwrap();
                    values[id][0] = if temp < values[id][0] {
                        temp
                    } else {
                        values[id][0]
                    };
                    values[id][1] = if temp > values[id][1] {
                        temp
                    } else {
                        values[id][1]
                    };
                    values[id][2] += 1;
                    values[id][3] += temp;
                }
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
    } else {
        println!("Could not open the file.");
    }

    // for (city, idx) in cities.into_iter() {
    //     println!(
    //         "{} {} {} {:.1}",
    //         city,
    //         values[idx][0] as f32 / 10.0,
    //         values[idx][1] as f32 / 10.0,
    //         values[idx][3] as f32 / values[idx][2] as f32 / 10.0,
    //     )
    // }
}
