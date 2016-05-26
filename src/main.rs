#![allow(dead_code)]

extern crate getopts;
extern crate rand;

use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::env;
use std::process;
use getopts::Options;
use rand::Rng;

// 解析参数
fn parse_args(args: &Vec<String>) -> (String, String, usize) {
    // 第一个参数为程序名
    let program = args[0].clone();

    // 构造参数
    let mut opts = Options::new();
    opts.optopt("n", "number", "number of line", "NUMBER");
    opts.optflag("h", "help", "show usage");

    // 解析参数
    let matches = opts.parse(&args[1..])
                      .ok()
                      .expect("Failed to parse args!");

    // 是否显示usage
    if matches.opt_present("h") {
        print_usage(&program, opts);
        process::exit(0);
    }

    // n 参数必须输入
    if ! matches.opt_present("n") {
        panic!("Options Format Error");
    } 

    let number = matches.opt_str("n");

    let num: usize = match number {
        Some(x) => x.trim()
                    .parse()
                    .ok()
                    .expect("Wrong Number"),
        None => {
            panic!("Options Format Error");
        },
    };

    // 输入的文件名
    let input_file = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        panic!("Options Format Error");
    };

    (program, input_file, num)
}

// 打印 usage
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage:: {} FILE [options]", program);
    println!("{}", opts.usage(&brief));
}

// 获取文件的总行数
fn get_line_number(input_file: &str) -> usize {
    let path = Path::new(input_file);

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("Error: {}\n", Error::description(&why)),
    };

    let reader = BufReader::new(file);

    reader.lines().count()
}

// 生成随机的行号
fn generate_random_numbers(total: usize, lines: usize) -> Vec<usize> {
    let mut random_numbers = vec![];
    for _ in 1..total {
        let mut rand_number = rand::thread_rng().gen_range(1, lines);
        while random_numbers.contains(&rand_number) {
            rand_number = rand::thread_rng().gen_range(1, lines);
        }
        random_numbers.push(rand_number);
    }
    random_numbers.sort();
    random_numbers

}

// 获取随机的行
fn achieve_lines(input_file: &str, number: usize, lines: usize) {
    let total = number + 1;

    // 生成随机的行号
    let random_numbers = generate_random_numbers(total, lines);

    // 打印相应行号对应的内容
    let path = Path::new(input_file);
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(why) => panic!("Error: {}\n", Error::description(&why)),
    };
    let reader = BufReader::new(file);

    let mut index = 0;
    let mut index_numbers = 0;
    
    for line in reader.lines() {
        index = index + 1;
        if index == random_numbers[index_numbers] {
            let line = match line {
                Ok(line) => line,
                Err(why) => {
                    panic!("Error: {}", Error::description(&why))
                },
            };
            println!("{}", line);
            index_numbers = index_numbers + 1;
            if index_numbers == random_numbers.len() {
                break;
            }
        }
    }
}

// 主函数
fn main() {
    // 获取所有的参数
    let args: Vec<String> = env::args().collect();

    // let (program, input_file, number) = parse_args(&args);
    // 解析参数
    let (_, input_file, number) = parse_args(&args);

    /*
    println!("program: {:?}", program);
    println!("input_file: {:?}", input_file);
    println!("number: {:?}", number);
    */

    let number_of_lines = get_line_number(&input_file);
    // println!("{:?}", number_of_lines);

    if number >= number_of_lines {
        println!("NUMBER is too big");
        return;
    }

    achieve_lines(&input_file, number, number_of_lines);
}

