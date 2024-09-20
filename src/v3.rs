use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn run_v3() {
    let mut cities: HashMap<String, usize> = HashMap::new();
    let mut values: [[f32; 4]; 10000] = [[0.0, 0.0, 0.0, 0.0]; 10000];
    let mut num_inserted = 0;

    let path = Path::new("measurements-10000000.txt");

    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(content) => {
                    let parts: Vec<&str> = content.splitn(2, ';').collect();
                    let name = parts[0];
                    let temp = parts[1].parse::<f32>().ok().unwrap();

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
                    values[id][2] += 1.0;
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
    //         values[idx][0],
    //         values[idx][1],
    //         values[idx][3] / values[idx][2],
    //     )
    // }
}
