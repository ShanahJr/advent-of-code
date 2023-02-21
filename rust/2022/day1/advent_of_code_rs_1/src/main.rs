use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./input.txt");
    let file = fs::read_to_string(path);

    match file {
        Ok(data) => {

            let calory_group: Vec<&str> = data.lines().collect();

            let mut calory_totals: Vec<u32> = Vec::new();
            let mut total = 0;

            for calory in calory_group.iter() {

                if !calory.is_empty() {
                    total = total + calory.trim().parse::<u32>().unwrap();
                } else {
                    calory_totals.push(total);
                    total = 0;
                }

            }

            let mut greatest_calory: u32 = 0;

            for total_calories in calory_totals.iter() {

                println!("{}", greatest_calory);
                println!("{}", total_calories);

                if greatest_calory < *total_calories  {
                    greatest_calory = *total_calories;
                }
                    
            }

            println!("{}",greatest_calory);

        },
        Err(err) => {
            println!("failed to open the input file, error: {}, the path is {}", err, {path.display()});
        },
    }

}
