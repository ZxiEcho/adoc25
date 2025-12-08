use std::{fs};

pub fn day3_p1() -> i32{
    let input = fs::read_to_string("test.txt").expect("error input");

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
        println!("{} {}",big_left as i32 - b'0' as i32, big_right as i32 - b'0' as i32 );

        (big_left as i32 - b'0' as i32) * 10 + (big_right as i32 - b'0' as i32)
        
    }).sum();

    return result;
}