#[macro_export]
macro_rules! tget {
    ($stack:expr) => {
        $stack.remove(0).0.unwrap()
    };

    ($stack:expr, $ty:ty) => {
        $stack.remove(0).1.unwrap().parse::<$ty>().unwrap()
    };
}

#[macro_export]
macro_rules! tignore {
    ($stack:expr) => {
        $stack.remove(0)
    };
}
