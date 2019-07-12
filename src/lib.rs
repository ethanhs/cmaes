//! A standard CMA-ES implementation with multithreaded support.
//! Implemented based on example code [here](en.wikipedia.org/wiki/CMA-ES).
//!
//! The algorithm minimizes the fitness function, so a lower fitness
//! represents a better individual.
//!
//! # Examples
//!
//! ```
//! use cmaes::*;
//!
//! #[derive(Clone)]
//! struct FitnessDummy;
//!
//! impl FitnessFunction for FitnessDummy {
//!     fn get_fitness(&self, parameters: &[f64]) -> f64 {
//!         // Calculate fitness here
//!
//!         0.0
//!     }
//! }
//!
//! // See the documentation for CMAESOptions for a list of all options
//! let options = CMAESOptions::default(2);
//!
//! // solution will be tuple with a set of optimized parameters, with as low a fitness as
//! // possible, and its fitness value
//! let solution = cmaes_loop(&FitnessDummy, options);
//! ```

extern crate la;
extern crate rand;

mod utils;
mod vector;
pub mod fitness;
pub mod cmaes_loop;
pub mod options;

pub use self::cmaes_loop::cmaes_loop_single;
pub use self::fitness::FitnessFunction;
pub use self::options::CMAESOptions;
