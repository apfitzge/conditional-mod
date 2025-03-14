/// Conditionally declares a mod as public.
macro_rules! conditional_pub_mod {
    ($name:ident, $cond:meta) => {
        #[cfg($cond)]
        pub mod $name;
        #[cfg(not($cond))]
        mod $name;
    };
}
