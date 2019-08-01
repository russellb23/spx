extern crate rand;
use std::char;
use std::collections::HashMap;

use rand::Rng;

fn unique(vec: Vec<usize>) -> Vec<usize> {
    let mut uniq: Vec<usize> = Vec::with_capacity(vec.len());
    let v = vec.clone();

    for i in v.iter() {
        if !uniq.contains(i) {
            uniq.push(*i);
        }
    }
    uniq.shrink_to_fit(); // useful under sparse cond
    uniq

}

fn basis_states(vec: Vec<usize>) -> Vec<String> {
    let mut bases: Vec<String> = Vec::new();
    let udaem: Vec<usize> = unique(vec.clone());

    for m in udaem.clone() {
        for n in udaem.clone() {
            let mut s = char::from_digit(m as u32, 10).unwrap().to_string();
            let t = char::from_digit(n as u32, 10).unwrap().to_string();
            s.push_str(t.as_str());
            if !bases.contains(&s) {
                bases.push(s)
            }
        }
    }
    bases.shrink_to_fit(); // useful under sparse condition
    bases
}

pub fn tranprob(vec: &Vec<usize>) -> HashMap<String, f32> {
    let trans_states: Vec<String> = basis_states(vec.clone());
    let mut pmat: HashMap<String, f32> = HashMap::new();

    for s in trans_states.iter() {
        let s = s.to_string();
        pmat.insert(s, 0.0f32);
    }

    let v = vec.clone();

    for p in 0..v.len() - 1 {
        let mut ss = char::from_digit(v[p] as u32, 10).unwrap().to_string();
        let tt = char::from_digit(v[p+1] as u32, 10).unwrap().to_string();

        ss.push_str(tt.as_str());
        if let Some(val) = pmat.get_mut(&ss) {
            *val += 1.0f32 / v.len() as f32;
        }

    }
    pmat
}
