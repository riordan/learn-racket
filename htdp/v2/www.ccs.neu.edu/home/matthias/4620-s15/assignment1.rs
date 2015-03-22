// ------------------------------------------------------------------
// determine the average of a sequence of rainfall measurements 

// ------------------------------------------------------------------
// the problem statements fails to specify what to produce for empty 
// sequences of measurements; the decision is encapsulated in 
//           RETURN_FOR_EMPTY_SEQUENCE
// ------------------------------------------------------------------

#![allow(unstable)]

use std::io::{BufferedReader};
use std::iter::AdditiveIterator;

// ------------------------------------------------------------------
static RETURN_FOR_EMPTY_SEQUENCE: f64 = 0f64;
static END : &'static str = "999";
// ------------------------------------------------------------------

#[cfg(not(test))]
fn main() {
    use std::io::stdin;
    use std::io::stdio::StdinReader;

    let in_port : BufferedReader<StdinReader> = 
        BufferedReader::new(stdin());
    let raw_data : Vec<f64>           = read_numbers(in_port);
    // what happens if we don't clone here? 
    let new_data : Vec<f64>           = raw_data.clone();
    let mean : f64                    = average(raw_data); 
    let (above,below) : (usize,usize) = above_below(mean,new_data);
    println!("{}\n{}\n{}\n",mean,above,below);
}

// ------------------------------------------------------------------
// read all non-negative numbers, one per line
// ignore all non-numbers and stop at 999.0 (comparing floats?)

#[cfg(test)]
mod read_numbers_examples {
    use std::io::{BufferedReader,MemReader};
    use super::read_numbers;

    #[test]
    fn tests() {
        // test a simple complete file 
        read_numbers_expect(
            "31.0\n20.0\n30.0\n40.0\n29.0", 
            vec![31f64,20f64,30f64,40f64,29f64]);
        // end sequence in 999 with additional numbers following 
        read_numbers_expect(
            "31.0\n20.0\n30.0\n40.0\n29.0\n999\n10\n20", 
            vec![31f64,20f64,30f64,40f64,29f64]);
        // ignore negative numbers 
        read_numbers_expect(
            "-31.0\n20.0\n30.0\n40.0\n29.0\n999\n10\n20", 
            vec![20f64,30f64,40f64,29f64]);
        // ignore random lines
        read_numbers_expect(
            "-31.0\n\n--&^%!--\n20.0\n30.0\n40.0\n29.0\n999\n10\n20", 
            vec![20f64,30f64,40f64,29f64]);
    }

    // auxiliary for reading a string into a vector, compare it to v
    fn read_numbers_expect(s: &str, v: Vec<f64>) {
        let bytes : Vec<u8> = s.to_string().into_bytes();
        let r : BufferedReader<MemReader> 
            = BufferedReader::new(MemReader::new(bytes));
        let x = read_numbers(r);
        assert_eq!(x, v);
    }
}

fn read_numbers<R : Reader>(mut s : BufferedReader<R>) -> Vec<f64> {
    let mut raw_data : Vec<f64> = vec![];
    for line in s.lines() {
        let u : String = line.unwrap();
        let l : &str = u.trim();
        if END == l {
            return raw_data;
        };
        let k : Option<f64> = l.parse();
        match k {
            Some(num) => 
                if num >= 0.0 {
                    raw_data.push(num);
                },
            None      => {}
        }
    };
    return raw_data
}

// ------------------------------------------------------------------
// determine the average of a vector of f64 numbers 
//
// NOTE: 
// this separate loop over the data helps testing but should probably 
// be fused explicitly with read_numbers if performance matters 
#[cfg(test)]
mod average_examples {
    use super::average;

    #[test]
    fn tests() {
        assert_eq!(average(vec![2.0,3.0]),2.5);
        assert_eq!(average(vec![]),0.0);
    }
}

fn average(raw_data : Vec<f64>) -> f64 {
    let l : f64 = raw_data.len() as f64;
    if 0.0 == l {
       return RETURN_FOR_EMPTY_SEQUENCE;
    }
    else {
        return raw_data.into_iter().map(|x| x).sum() / l;
    }
}

// ------------------------------------------------------------------
// how many points are in [average-5,average] and [average,average+5]
#[cfg(test)]
mod above_below_examples {
    use super::above_below;

    #[test]
    fn tests() {
        let v1 : Vec<f64> = vec![31f64,20f64,30f64,40f64,29f64];
        assert_eq!(above_below(30f64,v1),(2,2));
        assert_eq!(above_below(0f64,vec![]),(0,0));
    }
}

fn above_below(mean : f64, measurements : Vec<f64>) -> (usize,usize) {
    let mut above : usize = 0us; 
    let mut below : usize = 0us; 
    let add5 : f64 = mean + 5f64;
    let sub5 : f64 = mean - 5f64;
    // use single loop to process all measurements 
    for &x in measurements.iter() {
        if within(mean,x,add5) {
            above += 1;
        };
        if within(sub5,x,mean) {
            below += 1;
        };
    };

    return (above,below);
}

fn within(low : f64, x : f64, high : f64) -> bool {
    return low <= x && x <= high;
}
