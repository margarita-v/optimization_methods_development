extern crate optimization_methods_development;

use optimization_methods_development::dialogs::{user_input, input2f64};
use optimization_methods_development::methods::{segment_division, golden_section};
use optimization_methods_development::functions::{sample_func, sample_func_str};

fn main() {
    let input = user_input("Введите:\n\
                            1. Метод деления отрезка пополам \n\
                            2. Метод золотого сечения\n");

    match input.trim().parse::<i32>() {
        Ok(x) => {
            if x == 1 || x == 2 {
                let mut a : f64 = 0.0;
                let mut b : f64 = 0.0;
                            
                let mut input = user_input("Введите a (левая граница):");
                input2f64(&input, &mut a);	
            
                input = user_input("Введите b (правая граница):");
                input2f64(&input, &mut b);	

                let eps : f64 = 0.000001;
                let mut result : f64;
                if x == 1 {
                    result = segment_division::segment_divide(a, b, eps, &sample_func)
                } else {
                    result = golden_section::golden_section(a, b, eps, &sample_func)
                }

                println!("Точка минимума функции {} на заданном отрезке равна {:?}", 
                        sample_func_str(), 
                        result);
            }  else {
                println!("Введено некорректное число!");
            }
        }        
        Err(..) => println!("Не введено целое число!!"),
    }
}