use std::fs;

fn main() {
    let file_path="input.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    
    let highest_calories =  get_highest_calories(&input);
    println!("max calories: {}", highest_calories);
}

fn get_highest_calories(input: &str)-> u32{
    let mut sum: u32 = 0;
    let mut elfs_with_calories: Vec<u32> = vec![];
    let v = input.split('\n');
    //println!("{:?}", v);
    let mut x: u32; 
    for val in v {
        x = val.parse().unwrap_or(0);
        //println!("{:?}", x);
        if x != 0{
            sum = sum + x;
        }
        else{
            elfs_with_calories.push(sum);
            sum = 0
        }

    }
    elfs_with_calories.sort();
    return elfs_with_calories[elfs_with_calories.len()-1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works(){
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;
        assert_eq!(get_highest_calories(input), 24000);
    }
}

