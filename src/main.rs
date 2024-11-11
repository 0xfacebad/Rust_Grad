#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
extern crate ndarray;
extern crate rand;
use ndarray::Array1;
use rand::Rng;
fn mean_square_erro(arr:&Array1<i32> , arr1:&Array1<i32>)->f32{
        let n = arr.len() as f32;
        arr.iter()
        .zip(arr1.iter())
        .map(|(a,b)|{
            let diff = (*a-*b) as f32;
            diff *diff
        }).sum::<f32>()/n
}

fn gradient_descent(arr:&mut Array1<i32> , 
    arr1: &mut Array1<i32> ,
    iterations:usize,learning_rate:f32){
         let n = arr.len() as f32;
         for i in 0..iterations{
             let mut grad_arr = Array1::from_vec(vec![0.0;arr.len()]);
             for i in 0..arr.len(){
                  let diff = arr[i] as f32 - arr1[i] as f32;
                  grad_arr[i] = 2.0*(diff/n);}
             for i in 0..arr.len(){
                   arr[i] = (arr[i] as f32 - learning_rate * grad_arr[i]) as i32;
             }
             let loss = mean_square_erro(&arr , &arr1);
             println!("Iteraions: {} Loss: {}" , i+1 , loss);   
         }
}
fn main() {
   let size = 100;
   let mut rng = rand::thread_rng();
   // Created a numpy array of 1D with random values 
   // between 0 to 100 of 100 size
   let mut some_arr = Array1::from_iter((0..size).
   map(|_|rng.gen_range(0..100)));
   
   let mut another_arr = Array1::from_iter((0..size).map(|_|rng.gen_range(0..100)));
   let learning_rate = 0.01;
   let iterations = 100;
   gradient_descent(&mut some_arr, &mut another_arr, iterations, learning_rate);
   println!("{:#?}" , some_arr);
}
