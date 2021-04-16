use std::error::Error;
use std::collections::{HashSet, HashMap};
use std::time;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufWriter, BufReader};
use std::path::Path;
use std::io::{self, Read};

#[macro_use]
extern crate num_derive;

pub mod rng;
use crate::rng::*;

pub mod thrill_digger;
use crate::thrill_digger::*;

pub mod solver;
use crate::solver::*;
