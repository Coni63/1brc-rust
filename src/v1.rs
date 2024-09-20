use std::{
    collections::HashMap,
    fs::File,
    hash::{Hash, Hasher},
    io::{BufRead, BufReader},
    path::Path,
};

pub struct City {
    name: String,
    min_temp: f32,
    max_temp: f32,
    sum_temp: f32,
    count: i32,
}

impl City {
    pub fn new(name: String) -> City {
        City {
            name,
            min_temp: 0.0,
            max_temp: 0.0,
            sum_temp: 0.0,
            count: 0,
        }
    }

    pub fn add_temp(&mut self, temperature: f32) {
        self.count += 1;
        self.sum_temp += temperature;
        if temperature < self.min_temp {
            self.min_temp = temperature;
        }
        if temperature > self.max_temp {
            self.max_temp = temperature;
        }
    }

    pub fn get_result(&self) {
        println!(
            "{} {} {} {:.1}",
            self.name,
            self.min_temp,
            self.max_temp,
            self.sum_temp / self.count as f32,
        )
    }
}

impl Hash for City {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for City {
    fn eq(&self, other: &City) -> bool {
        self.name == other.name
    }
}

impl Eq for City {}

pub fn run_v1() {
    let mut cities: HashMap<String, City> = HashMap::new();

    let path = Path::new("measurements-10000000.txt");

    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(content) => {
                    let parts: Vec<&str> = content.splitn(2, ';').collect();
                    let name = parts[0];
                    let temp = parts[1].parse::<f32>().ok().unwrap();

                    let city = cities
                        .entry(name.to_string())
                        .or_insert(City::new(name.to_string()));
                    city.add_temp(temp);
                }
                Err(e) => eprintln!("Error reading line: {}", e),
            }
        }
    } else {
        println!("Could not open the file.");
    }

    // for city in cities.values() {
    //     city.get_result();
    // }
}
