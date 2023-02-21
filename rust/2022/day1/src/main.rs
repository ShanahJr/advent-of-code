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

            let greatest_calory = get_greatest_calory(&calory_totals);

            println!("{}",greatest_calory);

            let sorted_calories = selection_sort(calory_totals);
            let mut top_3_calories_sum: u32 = 0; 
            
            for x in 0..3  {
                
                top_3_calories_sum = top_3_calories_sum + sorted_calories[x];
            };

            println!("{}", top_3_calories_sum)

        },
        Err(err) => {
            println!("failed to open the input file, error: {}, the path is {}", err, {path.display()});
        },
    }

    fn get_greatest_calory(calory_totals: &Vec<u32>) -> u32 {
        let mut greatest_calory: u32 = 0;

        for total_calories in calory_totals.iter() {

            if greatest_calory < *total_calories  {
                greatest_calory = *total_calories;
            }
                
        }

        return greatest_calory;
    }

    fn selection_sort(calory_totals: Vec<u32>) -> Vec<u32> {
        
        let mut temp_vec = calory_totals.to_vec();

        for outer in 0..temp_vec.len()-1 {

            for inner in outer+1..temp_vec.len() {
                if temp_vec[outer] < temp_vec[inner] {
                    temp_vec = swap_values(temp_vec, outer, inner)
                }
            }
        }

        return temp_vec;

    }

    fn swap_values(mut calories:Vec<u32>, index1: usize ,  index2: usize) -> Vec<u32> {
        let temp: u32;

        temp = calories[index1];
        calories[index1] = calories[index2];
        calories[index2] = temp;

        return calories;

    }

}
