use std::{fs};

pub fn day3_p1() -> i32{
    let input = fs::read_to_string("test.txt").expect("error input");
    let mut result = 0;

    let _ = input.lines().map(|v|{
        let mut big_left = 0;
        let mut big_right = 0;
        let mut index_left = 0;
        let mut index_right = v.len();
        let line = v.as_bytes();
        let mut j = v.len() - 1; 
        for i in 0..v.len(){
            if line[i] > big_left && i < index_right{
                big_left = line[i];
                index_left = i;
            }
            if line[j] > big_right && j > index_left{
                big_right = line[j];
                index_right = j;
            }
            j -= 1;
        }

        result = (big_left - b'0') as i32 * 10 + (big_right - b'0') as i32;
        println!("{:?}",result);
    
    });

    return result;
}