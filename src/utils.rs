pub(crate) fn norm_2(v: &[f64]) -> f64 {
    let mut res = 0.0;

    for &v_i in v {
        res += v_i.abs().powf(2.0);
    }

    res.sqrt()
}

#[cfg(test)]
mod tests {
    use crate::utils;

    #[test]
    fn norm() {
        let norm = utils::norm_2(&vec![3., -2., 6.]);
        assert_eq!(norm, 7.0, "||(3, -2, 6)||_2 = 7");
    }
}
