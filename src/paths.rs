//! Contains some standard paths.

/// Return the path of the `discriminant` function, that is `::std::mem::discriminant`.
pub fn discriminant_path() -> syn::Path {
    if cfg!(feature = "use_core") {
        parse_quote!(::core::mem::discriminant)
    } else {
        parse_quote!(::std::mem::discriminant)
    }
}

/// Return the path of the `unreachable_unchecked` function, that is `::std::hint::unreachable_unchecked`.
pub fn unreachable_path() -> syn::Path {
    if cfg!(feature = "use_core") {
        parse_quote!(::core::hint::unreachable_unchecked)
    } else {
        parse_quote!(::std::hint::unreachable_unchecked)
    }
}