#![deny(
    absolute_paths_not_starting_with_crate,
    ambiguous_associated_items,
    anonymous_parameters,
    arithmetic_overflow,
    array_into_iter,
    asm_sub_register,
    bad_asm_style,
    bindings_with_variant_name,
    break_with_label_and_loop,
    byte_slice_in_packed_struct_with_derive,
    cenum_impl_drop_cast,
    clashing_extern_declarations,
    coherence_leak_check,
    conflicting_repr_hints,
    confusable_idents,
    const_evaluatable_unchecked,
    const_item_mutation,
    dead_code,
    deprecated,
    deprecated_cfg_attr_crate_type_name,
    deprecated_in_future,
    deprecated_where_clause_location,
    deref_into_dyn_supertrait,
    deref_nullptr,
    drop_bounds,
    duplicate_macro_attributes,
    dyn_drop,
    ellipsis_inclusive_range_patterns,
    enum_intrinsics_non_enums,
    explicit_outlives_requirements,
    exported_private_dependencies,
    for_loops_over_fallibles,
    forbidden_lint_groups,
    function_item_references,
    ill_formed_attribute_input,
    illegal_floating_point_literal_pattern,
    implied_bounds_entailment,
    improper_ctypes,
    improper_ctypes_definitions,
    incomplete_features,
    incomplete_include,
    indirect_structural_match,
    ineffective_unstable_trait_impl,
    inline_no_sanitize,
    invalid_atomic_ordering,
    invalid_doc_attributes,
    invalid_type_param_default,
    invalid_value,
    irrefutable_let_patterns,
    keyword_idents,
    large_assignments,
    late_bound_lifetime_arguments,
    legacy_derive_helpers,
    let_underscore_drop,
    let_underscore_lock,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_docs,
    missing_fragment_specifier,
    mixed_script_confusables,
    mutable_transmutes,
    named_arguments_used_positionally,
    named_asm_labels,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_ascii_idents,
    non_camel_case_types,
    non_fmt_panics,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    nontrivial_structural_match,
    noop_method_call,
    opaque_hidden_inferred_bound,
    order_dependent_trait_objects,
    overflowing_literals,
    overlapping_range_endpoints,
    path_statements,
    patterns_in_fns_without_body,
    pointer_structural_match,
    private_in_public,
    proc_macro_back_compat,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    redundant_semicolons,
    repr_transparent_external_private_fields,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    semicolon_in_expressions_from_macros,
    soft_unstable,
    special_module_name,
    stable_features,
    suspicious_auto_trait_impls,
    temporary_cstring_as_ptr,
    text_direction_codepoint_in_comment,
    text_direction_codepoint_in_literal,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    uncommon_codepoints,
    unconditional_panic,
    unconditional_recursion,
    unexpected_cfgs,
    ungated_async_fn_track_caller,
    uninhabited_static,
    unknown_crate_types,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unstable_name_collisions,
    unstable_syntax_pre_expansion,
    unsupported_calling_conventions,
    unused_allocation,
    unused_assignments,
    unused_assignments,
    unused_attributes,
    unused_braces,
    unused_comparisons,
    unused_crate_dependencies,
    unused_doc_comments,
    unused_extern_crates,
    unused_features,
    unused_import_braces,
    unused_imports,
    unused_labels,
    unused_lifetimes,
    unused_macro_rules,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_tuple_struct_fields,
    unused_unsafe,
    unused_variables,
    useless_deprecated,
    where_clauses_object_safety,
    while_true
)]
#![warn(macro_use_extern_crate, unknown_lints)]
#![allow(
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths,
    missing_debug_implementations,
    single_use_lifetimes,
    unused_results,
    variant_size_differences,
    warnings,
    renamed_and_removed_lints
)]

//! # rust_info
//!
//! Extracts and provides the current rust compiler information.
//!
//! This library main goal is to provide development/build tools such as
//! [cargo-make](https://sagiegurari.github.io/cargo-make/)the needed information on the current rust installation and
//! setup.
//!
//! # Examples
//!
//! ```
//! fn main() {
//!     let rust_info = rust_info::get();
//!
//!     println!("Version: {}", rust_info.version.unwrap());
//!     println!("Channel: {:#?}", rust_info.channel.unwrap());
//!     println!("Target Arch: {}", rust_info.target_arch.unwrap_or("unknown".to_string()));
//!     println!("Target Env: {}", rust_info.target_env.unwrap_or("unknown".to_string()));
//!     println!("Target OS: {}", rust_info.target_os.unwrap_or("unknown".to_string()));
//!     println!("Target Pointer Width: {}", rust_info.target_pointer_width.unwrap_or("unknown".to_string()));
//!     println!("Target Vendor: {}", rust_info.target_vendor.unwrap_or("unknown".to_string()));
//!     println!("Target Triple: {}", rust_info.target_triple.unwrap_or("unknown".to_string()));
//! }
//! ```
//!
//! # Installation
//! In order to use this library, just add it as a dependency:
//!
//! ```ini
//! [dependencies]
//! rust_info = "*"
//! ```
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/rust_info/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/rust_info/blob/master/LICENSE) open source license.
//!

#[cfg(test)]
#[path = "./lib_test.rs"]
mod lib_test;

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

mod rustinfo;
mod triple;
pub mod types;

use std::{borrow::Cow, path::Path};

use crate::types::RustInfo;

/// Options used for querying rust info
#[derive(Debug, Clone)]
pub struct Options<'path> {
    /// Optionally override working directory used for querying `rustc`
    pub path: Option<Cow<'path, Path>>,
}

impl<'path> Default for Options<'path> {
    fn default() -> Self {
        Self { path: None }
    }
}

impl<'path> Options<'path> {
    /// Helper for calling `default`
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the `path` option
    pub fn path(mut self, path: Option<Cow<'path, Path>>) -> Self {
        self.path = path;
        self
    }
}

/// Loads and returns the current rust compiler version and setup.<br>
/// In case partial data is not available, those values will be set to Option::None.
///
/// # Example
///
/// ```
/// fn main() {
///     let rust_info = rust_info::get();
///
///     println!("Version: {}", rust_info.version.unwrap());
///     println!("Channel: {:#?}", rust_info.channel.unwrap());
///     println!("Target Arch: {}", rust_info.target_arch.unwrap_or("unknown".to_string()));
///     println!("Target Env: {}", rust_info.target_env.unwrap_or("unknown".to_string()));
///     println!("Target OS: {}", rust_info.target_os.unwrap_or("unknown".to_string()));
///     println!("Target Pointer Width: {}", rust_info.target_pointer_width.unwrap_or("unknown".to_string()));
///     println!("Target Vendor: {}", rust_info.target_vendor.unwrap_or("unknown".to_string()));
///     println!("Target Triple: {}", rust_info.target_triple.unwrap_or("unknown".to_string()));
/// }
/// ```
pub fn get() -> RustInfo {
    get_with_options(&Options::default())
}

/// Loads and returns the current rust compiler version and setup for a specified path.<br>
/// In case partial data is not available, those values will be set to Option::None.
///
/// # Example
///
/// ```
/// fn main() {
///     let opts = rust_info::Options::new().path(Some(std::path::Path::new("./").into()));
///     let rust_info = rust_info::get_with_options(&opts);
///
///     println!("Version: {}", rust_info.version.unwrap());
///     println!("Channel: {:#?}", rust_info.channel.unwrap());
///     println!("Target Arch: {}", rust_info.target_arch.unwrap_or("unknown".to_string()));
///     println!("Target Env: {}", rust_info.target_env.unwrap_or("unknown".to_string()));
///     println!("Target OS: {}", rust_info.target_os.unwrap_or("unknown".to_string()));
///     println!("Target Pointer Width: {}", rust_info.target_pointer_width.unwrap_or("unknown".to_string()));
///     println!("Target Vendor: {}", rust_info.target_vendor.unwrap_or("unknown".to_string()));
///     println!("Target Triple: {}", rust_info.target_triple.unwrap_or("unknown".to_string()));
/// }
/// ```
pub fn get_with_options(options: &Options) -> RustInfo {
    rustinfo::get(options)
}
