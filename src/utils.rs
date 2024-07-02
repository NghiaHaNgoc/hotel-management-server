use std::{collections::HashSet, hash::Hash, str::FromStr};

use anyhow::anyhow;
use chrono::{DateTime, Utc};

pub fn vector_difference<T: Clone + Eq + Hash>(v1: &Vec<T>, v2: &Vec<T>) -> Vec<T> {
    let s1: HashSet<T> = v1.iter().cloned().collect();
    let s2: HashSet<T> = v2.iter().cloned().collect();
    (&s1 - &s2).iter().cloned().collect()
}

pub fn valid_time_from_and_to(from: &str, to: &str) -> Result<bool, anyhow::Error> {
    let time_from: DateTime<Utc> = DateTime::from_str(from)?;
    let time_to: DateTime<Utc> = DateTime::from_str(to)?;

    if time_from >= time_to {
        Ok(false)
    } else {
        Ok(true)
    }
}

pub fn is_ovelaped_date_range(a: (&str, &str), b: (&str, &str)) -> Result<bool, anyhow::Error> {
    let a_from: DateTime<Utc> = DateTime::from_str(a.0)?;
    let a_to: DateTime<Utc> = DateTime::from_str(a.1)?;
    let b_from: DateTime<Utc> = DateTime::from_str(b.0)?;
    let b_to: DateTime<Utc> = DateTime::from_str(b.1)?;

    if a_from >= a_to || b_from >= b_to {
        return Err(anyhow!("Time from is equal or greater than time to."));
    }

    if a_to <= b_from || a_from >= b_to {
        Ok(false)
    } else {
        Ok(true)
    }
}
