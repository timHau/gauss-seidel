pub(crate) fn dist_2(v: &[f64], w: &[f64]) -> f64 {
    assert_eq!(
        v.len(),
        w.len(),
        "v, w should come from the same Vectorspace"
    );

    let mut res = 0.0;

    for (&v_i, &w_i) in v.iter().zip(w.iter()) {
        res += (v_i - w_i).abs().powf(2.0);
    }

    res.sqrt()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::utils;

    #[test]
    fn norm() {
        let dist = utils::dist_2(&vec![2.0, -1.0], &vec![-2.0, 2.0]);
        assert_eq!(dist, 5.0, "| (2, -1).T, (-2, 2).T |_2 = 5");
    }
}
