pub fn euclidean<T>(a: &T, b: &T) -> f64
where
    T: Into<f64> + Copy,
{
    let a: f64 = (*a).into();
    let b: f64 = (*b).into();

    (a - b).powi(2).sqrt()
}
