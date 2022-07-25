use std::io;
use std::io::prelude::*;
use std::fmt::Debug;
use std::collections::HashMap;

fn main() {
    let mut inputs = String::new();
    for _ in 0..3 {
        io::stdin().read_line(&mut inputs).unwrap();
    }
    let new_inputs = inputs.split("\n");
    let mut new_inputs = new_inputs.collect::<Vec<_>>();
    new_inputs.pop();
    println!("hi, {:?}", new_inputs);

    let nodes: u32 = new_inputs[1].parse().unwrap();

    let mut data = String::new();
    for _ in 0..nodes {
        io::stdin().read_line(&mut data).unwrap();
    }
    let new_data = data.split("\n");
    let mut new_data = new_data.collect::<Vec<_>>();
    new_data.pop();
    println!("hi, {:?}", new_data);

    let queries: u32 = new_inputs[2].parse().unwrap();

    let mut query_data = String::new();
    for _ in 0..queries {
        io::stdin().read_line(&mut query_data).unwrap();
    }
    let new_queries = query_data.lines().collect::<Vec<_>>();
    println!("hi, {:?}", new_queries);
}
