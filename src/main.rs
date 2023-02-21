
#![allow(unused)]
use std::io;
use rand::Rng;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    /*println!("What is your name?");
    let mut name:String = String::new();
    let greeting="Nice to meet you";
    io::stdin().read_line(&mut name)
        .expect("Didn't Receive Input");
    println!("Hello {}! {}",name.trim_end(),greeting);*/
    const ONE_MIL:u32=1_000_000;
    const PI: f32 = 3.1211592;
    let age="47";
    let mut age:u32=age.trim().parse()
        .expect("Age wan´t assigned a number");
    age = age +1;
    println!("I'm {} and I want ${}",age, ONE_MIL);
    println!("Max u32 : {}",u32::MAX);
    let is_true =true;
    let my_grade = 'a';
    let num_2=5;
    let num_3 =6;
    println!("5 + 6 = {}",num_2+num_3);
    let random_num=rand::thread_rng()
        .gen_range(1..11);
    println!("Random : {}",random_num);
    let age =8;
    if(age>=1)&&(age<=18){
        print!("Your not adult");
    }else if age ==21||age==50{
        println!("Your not a teenager");
    }else if age >= 65 {
        println!("Important Birthday");
    }else{
        print!("The Birthday is not important");
    }
}
