use std::{fs, i64};

pub fn day5_p1() -> i32{
    let file = fs::read_to_string("day5.txt").expect("kass");
    let mut return_value = 0;

    let both_input = file.split("\n\n").collect::<Vec<_>>();
    let lines:Vec<&str> = both_input[0].lines().collect();
    let compare_values = both_input[1].lines().collect::<Vec<&str>>();

    let range = lines.iter().map(|v| v.split_once("-").unwrap()).collect::<Vec<(&str, &str)>>();

    for i in compare_values{
        let compare = i.parse::<i64>().unwrap();

        for j in 0..range.len(){
            let (start, end) = range[j].clone();
            let first:i64 = start.parse().unwrap();
            let last:i64 = end.parse().unwrap();
            if compare < first || compare > last {
                continue;
            }
            else {
                return_value += 1;
                break;
            }
        }
        
    }
    return return_value;
}

pub fn day5_p2() -> i64{
    let file = fs::read_to_string("day5.txt").expect("kass");

    let both_input = file.split("\n\n").collect::<Vec<_>>();
    let lines:Vec<&str> = both_input[0].lines().collect();
    
    let range = lines.iter().map(|v| v.split_once("-").unwrap()).collect::<Vec<(&str, &str)>>();
    let mut fresh_food:Vec<(i64, i64)> = Vec::new(); 
    
    for i in 0..range.len(){
        let mut indexes:Vec<usize> = Vec::new();
        let (start, end) = range[i].clone();
        let first:i64 = start.parse().unwrap();
        let last:i64 = end.parse().unwrap();

        if i == 0{
            fresh_food.push((first,last));
            continue;
        }
        else {
            add_to_vector(&mut fresh_food, first, last, &mut indexes);
        }
        for j in 0..indexes.len(){
            //println!("{:?} {:?} {:?}", j, fresh_food, indexes);
            fresh_food.remove(indexes[j]);
            
            
            for k in 0..indexes.len(){
                if indexes[k] > j{
                    indexes[k] -= 1;
                }
            }
        }

    }

    let mut return_value:i64 = 0;
    for (i, j) in &fresh_food{
        return_value += j - i + 1;
    }

    println!("{:?}", fresh_food);


    return return_value as i64;
}

fn add_to_vector(fresh_food: &mut Vec<(i64, i64)>, first: i64, last: i64, indexes: &mut Vec<usize>){
    let mut added = true;

    for j in 0..fresh_food.len(){
        added = true;
        let (mut start, mut end) = fresh_food[j];
        if first < end && first > start && last > end{
            end = last;
            //println!("{} {:?}", j, fresh_food);
            indexes.push(j);
            add_to_vector(fresh_food, start, end, indexes);
            break;
        }
        else if last < end && last > start && first < start{
            start = first;
            //println!("{} {:?}", j, fresh_food);
            indexes.push(j);
            add_to_vector(fresh_food, start, end, indexes);
            break;
        }
        else {
            added = false;
        }
    }
    if !added && !fresh_food.contains(&(first,last)){
        fresh_food.push((first,last));
    }
    
}