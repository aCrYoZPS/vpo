use core::f64;
use std::fmt::Debug;
use std::{fmt, io};

#[derive(Debug)]
pub struct FloatError {
    message: String,
}

impl fmt::Display for FloatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

pub fn input_f64() -> Result<f64, FloatError> {
    let mut input_buffer = String::new();
    match io::stdin().read_line(&mut input_buffer) {
        Ok(_) => (),
        Err(err) => {
            return Err(FloatError {
                message: format!("{}", err),
            });
        }
    };

    return match input_buffer.trim().parse::<f64>() {
        Ok(f) => Ok(f),
        Err(err) => Err(FloatError {
            message: format!("{}", err),
        }),
    };
}

pub fn calculate_area(a: f64, b: f64) -> Result<f64, FloatError> {
    if a <= 0.0 || b <= 0.0 {
        return Err(FloatError {
            message: "A rectangle cannot have side of length zero or of negative length"
                .to_string(),
        });
    }

    let result = a * b;
    if result.is_infinite() {
        return Err(FloatError {
            message: "Result infinite".to_string(),
        });
    } else if result.is_nan() {
        return Err(FloatError {
            message: "Result is NaN".to_string(),
        });
    }

    return Ok(result);
}
