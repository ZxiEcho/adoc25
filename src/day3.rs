use std::{fs};

pub fn day3_p1() -> i32{
    let input = fs::read_to_string("day3.txt").expect("error input");

    let result = input.lines().map(|v|{
        let mut big_left = 0;
        let mut big_right = 0;
        let mut index_left = 0;
        let mut index_right = v.len();
        let mut reserv = 0;
        let mut reserv_id = v.len();
        let line = v.as_bytes();
        let mut j = v.len(); 
        for i in 0..v.len() - 1{
            j -= 1;
            if line[i] > big_left && i < index_right{
                big_left = line[i];
                index_left = i;

            }
            if line[j] > big_right && j > index_left{
                reserv = big_right;
                //println!("{}", reserv as i32 - b'0' as i32);
                reserv_id = index_right;
                big_right = line[j];
                //println!("{}", big_right as i32 - b'0' as i32);
                index_right = j;
            }
            if line[i] > big_left && big_right > big_left && i == index_right{
                big_left = line[i];
                index_left = i;
                big_right = reserv;
                index_right = reserv_id;
                
            }
        }
        //println!("{} {}",big_left as i32 - b'0' as i32, big_right as i32 - b'0' as i32 );

        (big_left as i32 - b'0' as i32) * 10 + (big_right as i32 - b'0' as i32)
        
    }).sum();

    return result;
}

pub fn day3_p2() -> i128{
    let input = fs::read_to_string("day3.txt").expect("error input");

    let result:i128 = input.lines().map(|v|{
        let mut batteries = Vec::new();
        let line = v.as_bytes();
        let mut final_ = 0;
        let mut index = 0;
        for i in 0..12{
           let mut current_big = 0;
           for j in  index..((v.len() + 1) - (12 - i)){
            //println!("{:?} line j",line[j] - b'0');
              if line[j] > current_big {
                current_big = line[j];
                index = j + 1;
                //println!("hit");
              }
            }
            //println!("{}",current_big - b'0');
            batteries.push(current_big - b'0');
            //println!("{:?}",batteries);
        }
        let mut j = 0;
        for i in (0..batteries.len()).rev(){
            //println!("{}", i);
            let power:i128 = 10i128.pow(i as u32) as i128;
            //println!("{}",power);
            final_ += ((batteries[j]) as i128 * power) as i128;
            j += 1;
            //println!("efter final_")
        }
        println!("{:?}", final_);
        final_
    }).sum();

    return result;
}