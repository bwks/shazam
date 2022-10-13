/// Convert a vector of str to a vector of String
#[macro_export]
macro_rules! string_vec {
    ($($x:expr),*) => (vec![$($x.to_owned()),*]);
}
