# conditional-mod

Provides a simple macro `conditional_vis_mod` to change module visibility based on a condition.

## Usage

Assume there is some module file `my_module.rs`.
The visibility of the module can be changed based on conditions, such as compiler features:

```rust
// This will declare the module as public if "my_feature" is active, and private otherwise.
conditional_mod::conditional_vis_mod!(my_module, feature = "my_feature", pub);

// This will declare the module as public if "my_feature" is not active, and `pub(crate)` otherwise.
conditional_mod::conditional_vis_mod!(my_module, feature = "my_feature", pub, pub(crate));
```
