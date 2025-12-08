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
        for i in 0..v.len(){
            j -= 1;
            if line[i] > big_left && i < index_right{
                big_left = line[i];
                index_left = i;

            }
            if line[j] > big_right && j > index_left{
                reserv = big_right;
                reserv_id = index_right;
                big_right = line[j];
                index_right = j;
            }
            if line[i] > big_left && big_right > big_left && i == index_right{
                big_left = line[i];
                index_left = i;
                big_right = reserv;
                index_right = reserv_id;
            }
        }

        (big_left - b'0') as i32 * 10 + (big_right - b'0') as i32
        
    }).sum();

    return result;
}