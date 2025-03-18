/// Conditionally declares a mod as public.
#[macro_export]
macro_rules! conditional_vis_mod {
    ($name:ident, $cond:meta, $conditional_vis:vis) => {
        #[cfg($cond)]
        $conditional_vis mod $name;
        #[cfg(not($cond))]
        mod $name;
    };
    ($name:ident, $cond:meta, $conditional_vis:vis, $fallback_vis:vis) => {
        #[cfg($cond)]
        $conditional_vis mod $name;
        #[cfg(not($cond))]
        $fallback_vis mod $name;
    };
}
