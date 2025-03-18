/// Conditionally declares a mod as public.
///
/// Arguments:
/// - name - The name of the module being declared
/// - cond - The condition for `conditional_vis`
/// - conditional_vis - The visibility to use if `cond` is true
/// - fallback_vis - Optional visibility to use if `cond` is false
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
