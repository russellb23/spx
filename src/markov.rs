use std::char;
use std::collections::HashMap;

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

fn basis_states(vec: Vec<usize>, norder: usize) -> Vec<String> {
    let mut bases: Vec<String> = Vec::new();
    let udaem: Vec<usize> = unique(vec.clone());

    match norder {
        2 => {
            for m in udaem.clone() {
                for n in udaem.clone() {
                    let mut s = char::from_digit(m as u32, 10).unwrap().to_string();
                    let t = char::from_digit(n as u32, 10).unwrap().to_string();
                    println!("Defaults: {}", &s);
                    s.push_str(t.as_str());
                    if !bases.contains(&s) {
                        bases.push(s)
                    }
                }
            }
        }
        3 => {
            for m in udaem.clone() {
                for n in udaem.clone() {
                    for p in udaem.clone() {
                        let mut s = char::from_digit(m as u32, 10).unwrap().to_string();
                        let mut t = char::from_digit(n as u32, 10).unwrap().to_string();
                        let u = char::from_digit(p as u32, 10).unwrap().to_string();
                        t.push_str(u.as_str());
                        s.push_str(&t);
                        if !bases.contains(&s) {
                            bases.push(s)
                        }
                    }
                }
            }
        }
        4 => {
            for m in udaem.clone() {
                for n in udaem.clone() {
                    for p in udaem.clone() {
                        for q in udaem.clone() {
                            let mut s = char::from_digit(m as u32, 10).unwrap().to_string();
                            let mut t = char::from_digit(n as u32, 10).unwrap().to_string();
                            let mut u = char::from_digit(p as u32, 10).unwrap().to_string();
                            let v = char::from_digit(q as u32, 10).unwrap().to_string();
                            u.push_str(v.as_str());
                            t.push_str(&u);
                            s.push_str(&t);
                            //s.push_str(t.push_str(u.push_str(v.as_str())));
                            if !bases.contains(&s) {
                                bases.push(s)
                            }
                        }
                    }
                }
            }
        }
        5 => {
            for m in udaem.clone() {
                for n in udaem.clone() {
                    for p in udaem.clone() {
                        for q in udaem.clone() {
                            for x in udaem.clone() {
                                let mut s = char::from_digit(m as u32, 10).unwrap().to_string();
                                let mut t = char::from_digit(n as u32, 10).unwrap().to_string();
                                let mut u = char::from_digit(p as u32, 10).unwrap().to_string();
                                let mut v = char::from_digit(q as u32, 10).unwrap().to_string();
                                let w = char::from_digit(x as u32, 10).unwrap().to_string();
                                v.push_str(w.as_str());
                                u.push_str(&v);
                                t.push_str(&u);
                                s.push_str(&t);
                                //s.push_str(t.push_str(u.push_str(v.push_str(w.as_str()))));
                                if !bases.contains(&s) {
                                    bases.push(s)
                                }
                            }
                        }
                    }
                }
            }
        }
        // Defaults to first order markov
        _ => {
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
        }
    }
    bases.shrink_to_fit(); // useful under sparse condition
    bases
}

pub fn tranprob(vec: &Vec<usize>, norder: usize) -> HashMap<String, f32> {
    let trans_states: Vec<String> = basis_states(vec.clone(), norder);
    let mut pmat: HashMap<String, f32> = HashMap::new();
    //let _data = vec.clone(); //String::from_utf8(vec.clone()).unwrap();

    for s in trans_states.iter() {
        let s = s.to_string();
        pmat.insert(s, 0.0f32);
    }

    let v = vec.clone();
    let n = v.len() as f32;

    for p in 0..v.len() - norder - 1 {
        match norder {
            2 => {
                let mut ss = char::from_digit(v[p] as u32, 10).unwrap().to_string();
                let tt = char::from_digit(v[p + 1] as u32, 10).unwrap().to_string();

                ss.push_str(tt.as_str());
                if let Some(val) = pmat.get_mut(&ss) {
                    *val += 1.0f32 / n;
                }
            }
            3 => {
                let mut ss = char::from_digit(v[p] as u32, 10).unwrap().to_string();
                let mut tt = char::from_digit(v[p + 2] as u32, 10).unwrap().to_string();
                let rr = char::from_digit(v[p + 3] as u32, 10).unwrap().to_string();
                tt.push_str(rr.as_str());
                ss.push_str(&tt);

                if let Some(val) = pmat.get_mut(&ss) {
                    *val += 1.0f32 / n;
                }
            }
            4 => {
                let mut ss = char::from_digit(v[p] as u32, 10).unwrap().to_string();
                let mut tt = char::from_digit(v[p + 1] as u32, 10).unwrap().to_string();
                let mut rr = char::from_digit(v[p + 2] as u32, 10).unwrap().to_string();
                let pp = char::from_digit(v[p + 3] as u32, 10).unwrap().to_string();
                rr.push_str(pp.as_str());
                tt.push_str(&rr);
                ss.push_str(&tt);

                if let Some(val) = pmat.get_mut(&ss) {
                    *val += 1.0f32 / n;
                }
            }
            5 => {
                let mut ss = char::from_digit(v[p] as u32, 10).unwrap().to_string();
                let mut tt = char::from_digit(v[p + 1] as u32, 10).unwrap().to_string();
                let mut rr = char::from_digit(v[p + 2] as u32, 10).unwrap().to_string();
                let mut pp = char::from_digit(v[p + 3] as u32, 10).unwrap().to_string();
                let qq = char::from_digit(v[p + 4] as u32, 10).unwrap().to_string();
                pp.push_str(qq.as_str());
                rr.push_str(&pp);
                tt.push_str(&rr);
                ss.push_str(&tt);

                if let Some(val) = pmat.get_mut(&ss) {
                    *val += 1.0f32 / n;
                }
            }
            _ => {
                let mut ss = char::from_digit(v[p] as u32, 10).unwrap().to_string();
                let tt = char::from_digit(v[p + 1] as u32, 10).unwrap().to_string();

                ss.push_str(tt.as_str());
                if let Some(val) = pmat.get_mut(&ss) {
                    *val += 1.0f32 / n;
                }
            }
        }
    }
    pmat
}
