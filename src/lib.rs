//! A Linear Algebra Library for Game Graphics and Physics

#![deny(
    absolute_paths_not_starting_with_crate,
    bad_style,
    dead_code,
    explicit_outlives_requirements,
    improper_ctypes,
    keyword_idents,
    let_underscore_drop,
    macro_use_extern_crate,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    noop_method_call,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unconditional_recursion,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_parens,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    while_true
)]

pub mod marker;
pub mod matrix;
pub mod ops;
pub mod vector;
