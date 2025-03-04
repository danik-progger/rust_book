// import Asparagus
use crate::garden::vegetables::Asparagus as Sparge;
use rand::{CryptoRng, ErrorKind::Transient, Rng}; // rand::CryptoRng, randErrorKind::Transient ...
use std::io::{self, Write}; // std::io, std::io::Write
use std::collections::*;

pub mod garden;

fn main() {
    let plant = Sparge {};
    println!("I'm growing {plant:?}!");
}
