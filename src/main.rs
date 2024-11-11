#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
extern crate ndarray;
extern crate rand;
use ndarray::Array1;
use rand::Rng;

fn mean_squared_error(arr: &Array1<f32>, arr1: &Array1<f32>) -> f32 {
    let n = arr.len() as f32;
    arr.iter()
        .zip(arr1.iter())
        .map(|(a, b)| {
            let diff = a - b;
            diff * diff
        })
        .sum::<f32>()
        / n
}

fn gradient_descent(arr: &mut Array1<f32>,
    arr1: &Array1<f32>,
    iterations: usize,
    learning_rate: f32) {
    
    let n = arr.len() as f32;
    for i in 0..iterations {
        let mut grad_arr = Array1::zeros(arr.len());
        // Compute the gradient
        for i in 0..arr.len() {
            let diff = arr[i] - arr1[i];
            grad_arr[i] = 2.0 * (diff / n);
        }

        // Update the parameters
        for i in 0..arr.len() {
            arr[i] -= learning_rate * grad_arr[i];
        }

        // Calculate and log loss every 10 iterations
        if i % 10 == 0 {
            let loss = mean_squared_error(arr, arr1);
            println!("Iteration: {} | Loss: {}", i + 1, loss);
        }
    }
}

fn main() {
    let size = 100;
    let mut rng = rand::thread_rng();
    
    // Create random data arrays of size 100
    let mut some_arr = Array1::from_iter((0..size).map(|_| rng.gen_range(0..100) as f32));
    let another_arr = Array1::from_iter((0..size).map(|_| rng.gen_range(0..100) as f32));

    let learning_rate = 0.01;
    let iterations = 100;
    
    // Perform gradient descent
    gradient_descent(&mut some_arr, &another_arr, iterations, learning_rate);
    
    println!("{:?}", some_arr);
}

