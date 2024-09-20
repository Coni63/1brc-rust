use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn run_v2() {
    let mut cities: HashMap<String, [f32; 4]> = HashMap::new();

    let path = Path::new("measurements-10000000.txt");

    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(content) => {
                    let parts: Vec<&str> = content.splitn(2, ';').collect();
                    let name = parts[0];
                    let temp = parts[1].parse::<f32>().ok().unwrap();

                    let values = cities.entry(name.to_string()).or_insert([0.0; 4]);

                    values[0] = if temp < values[0] { temp } else { values[0] };
                    values[1] = if temp > values[1] { temp } else { values[1] };
                    values[2] += 1.0;
                    values[3] += temp;
                }
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
    } else {
        println!("Could not open the file.");
    }

    // for (city, values) in cities.into_iter() {
    //     println!(
    //         "{} {} {} {:.1}",
    //         city,
    //         values[0],
    //         values[1],
    //         values[3] / values[2],
    //     )
    // }
}
