pub trait Field<T> {
    fn interaction(&self, site: &T) -> f64;
}
