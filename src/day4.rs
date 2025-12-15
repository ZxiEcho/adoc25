use std::fs;

#[derive(Debug, PartialEq)]
enum Paper {
    Set,
    Unset,
}

pub fn day4_p1() -> i32 {
    let file = fs::read_to_string("day4.txt").expect("kass");
    let mut result = 0;

    let map: Vec<Vec<Paper>> = file
        .lines()
        .map(|v| {
            v.chars()
                .map(|w| match w {
                    '@' => Paper::Set,
                    '.' => Paper::Unset,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    
    for i in 0..map.len(){
        let mut counter = 0;
        for j in 0..map[i].len(){
            let item = collec_item(i as isize, j as isize, &map);
            let neighbours = find_neighbiurs(i,j, &map);
            match item {
                Paper::Set => {
                    if neighbours < 4{
                        counter += 1;
                    }
                },
                Paper::Unset => {}
            }
        }
        result += counter;
    }



    return result;
}

fn collec_item(i: isize, j:isize, map:&Vec<Vec<Paper>>) -> Paper{
    if i < 0 || i >= map.len() as isize{
        return Paper::Unset; 
    }
    else if j < 0 || j >= map[i as usize].len() as isize{
        return Paper::Unset;
    }
    let item = &map[i as usize][j as usize];
    match item {
        Paper::Set => return Paper::Set,
        Paper::Unset => return Paper::Unset
    }

}

fn find_neighbiurs(i: usize, j:usize, map:&Vec<Vec<Paper>>) -> i32{
    let neighbours = vec![
        (-1,-1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1)]
        .iter()
        .map(|(x,y)| collec_item(i as isize + x, j as isize + *y, &map))
        .collect::<Vec<Paper>>().iter().filter(|w| **w == Paper::Set).count() as i32;

    neighbours

}

pub fn day4_p2() -> i32{
    let file = fs::read_to_string("day4.txt").expect("kass");
    let mut return_value = 0;

    let mut map: Vec<Vec<Paper>> = file
        .lines()
        .map(|v| {
            v.chars()
                .map(|w| match w {
                    '@' => Paper::Set,
                    '.' => Paper::Unset,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    loop {
        let this_itteration = day4_helper(&mut map);
        if this_itteration == 0{
            break;
        }
        return_value += this_itteration;
    }
    return return_value;
    
}


fn day4_helper(map:&mut Vec<Vec<Paper>>) -> i32{
    
    let mut result = 0;
    
    for i in 0..map.len(){
        let mut counter = 0;
        for j in 0..map[i].len(){
            let item = collec_item(i as isize, j as isize, &map);
            let neighbours = find_neighbiurs(i,j, &map);
            match item {
                Paper::Set => {
                    if neighbours < 4{
                        counter += 1;
                        map[i][j] = Paper::Unset;
                    }
                },
                Paper::Unset => {}
            }
        }
        result += counter;
    }



    return result;
}