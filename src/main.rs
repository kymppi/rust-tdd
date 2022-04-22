#![feature(io_read_to_string)]

use std::{
    error,
    fs::{read_to_string, write},
    path::Path,
    result,
};

type TResult<T> = result::Result<T, TError>;
type TError = Box<dyn error::Error>;

// Read file the file as a String
fn read_file(p: &str) -> TResult<String> {
    read_to_string(p).map_err(|e| e.into())
}

// Convert String to Vector of Numbers
fn split_numbers(s: &String) -> TResult<Vec<usize>> {
    s.split_whitespace()
        .map(|x| x.parse::<usize>().map_err(|e| e.into()))
        .collect()
}

// SUm all of the Numbers in together
fn add_numbers(v: Vec<usize>) -> usize {
    // or you can v.iter().sum()
    v.iter().fold(0, |mut sum, &x| {
        sum += x;
        sum
    })
}

// Write numbers to file
fn write_numbers(n: usize, p: &str) -> TResult<()> {
    let path = Path::new(p);
    let res = read_file(&path.display().to_string())?;

    write(path, format!("{}\n{}", res, n))?;

    Ok(())
}

fn main() -> TResult<()> {
    let path = "data/input.txt";
    let res = read_file(&path);

    match res {
        Ok(s) => {
            let v = split_numbers(&s)?;
            println!("{:?}", v);
            let sum = add_numbers(v);
            println!("{}", sum);
            write_numbers(sum, &path)?;
        }
        Err(_) => {}
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup_test_two_file() -> TResult<()> {
        write(Path::new("test_data/test_two.txt"), String::from("35\n9"))?;
        Ok(())
    }

    #[test]
    fn test_read_file() {
        let res = read_file("test_data/test_one.txt");
        assert!(res.is_ok());

        if let Ok(s) = res {
            assert_eq!(s, "3\n4\n5\n6")
        }
    }

    #[test]
    fn test_split_numbers() {
        let res = split_numbers(&String::from("3\n4\n9"));
        assert!(res.is_ok());

        if let Ok(v) = res {
            assert_eq!(v, vec![3, 4, 9])
        }
    }

    #[test]
    fn test_add_numbers() {
        let sum_sixteen = add_numbers(vec![3, 4, 9]);
        let sum_none = add_numbers(vec![]);

        assert!(sum_sixteen == 16);
        assert!(sum_none == 0);
    }

    #[test]
    fn test_write_numbers() {
        let res = setup_test_two_file();
        assert!(res.is_ok());

        let res = write_numbers(45, "test_data/test_two.txt");
        assert!(res.is_ok());

        let res = read_file("test_data/test_two.txt");

        if let Ok(s) = res {
            assert_eq!(s, "35\n9\n45")
        }
    }
}
