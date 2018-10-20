fn upper_bound<T, F>(v: &Vec<T>, f: &F) -> Option<usize>
    where F: Fn(&T) -> bool {
    if v.first().map(f) != Some(true) {
        return None;
    }

    let last_idx = (v.len() as i64 -1) as usize;
    if f(&v[last_idx]) {
        return Some(last_idx);
    }

    return Some(upper_bound(v, f, 0, last_idx));

    fn upper_bound<T, F>(v: &Vec<T>, f: &F, left: usize, right: usize) -> usize 
        where F: Fn(&T) -> bool {
        if left+1 == right {
            return left;
        }
        let m = (left + right) / 2;
        if f(&v[m]) {
            return upper_bound(v, f, m, right);
        } else {
            return upper_bound(v, f, left, m);
        }
    }
}

fn lower_bound<T, F>(v: &Vec<T>, f: &F) -> Option<usize>
    where F: Fn(&T) -> bool {
    if v.last().map(f) != Some(true) {
        return None;
    }

    if let Some(i) = upper_bound(v, &|x| !f(x)) {
        return Some(i + 1);
    } else {
        return Some(0);
    }
}
