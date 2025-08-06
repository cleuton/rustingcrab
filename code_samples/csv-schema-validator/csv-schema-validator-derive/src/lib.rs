extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, Meta, Expr, ExprLit, Lit, Ident,
};

// Structure to store field validations
struct FieldValidation {
    field_name: Ident,
    validations: Vec<Validation>,
}

// Enum representing the different types of validations that can be derived.
enum Validation {
    Range { min: f64, max: f64 },
    Regex { regex: String },
    Required,
    Custom { path: syn::Path }, // Expects a syn::Expr::Lit(syn::Lit::Str) which will be parsed as a Path
}

impl Validation {
    /// Parses the content of `#[validate(...)]` into a vector of `Validation`.
    fn parse_validations(input: syn::parse::ParseStream) -> syn::Result<Vec<Self>> {
        let mut validations = Vec::new();

        // Parses Meta items separated by commas (e.g., Path, NameValue, List)
        let meta_items = syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated(input)?;

        for meta in meta_items {
            match meta {
                // Handles `#[validate(required)]`
                Meta::Path(path) => {
                    if path.is_ident("required") {
                        validations.push(Validation::Required);
                    }
                }
                // Handles `#[validate(regex = "...")]` and `#[validate(custom = "...")]`
                Meta::NameValue(mnv) => {
                    if mnv.path.is_ident("regex") {
                        // In syn 2.0, value is an Expr
                        if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = mnv.value {
                            validations.push(Validation::Regex {
                                regex: lit_str.value(),
                            });
                        } else {
                            return Err(syn::Error::new_spanned(mnv.value, "Expected string literal for `regex`"));
                        }
                    } else if mnv.path.is_ident("custom") {
                        // The original expectation is a string literal (e.g., "validate_length").
                        // We will parse the string literal as a syn::Path.
                        if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = mnv.value {
                            let path: syn::Path = syn::parse_str(&lit_str.value())
                                .map_err(|e| syn::Error::new_spanned(lit_str, e))?;
                            validations.push(Validation::Custom { path });
                        } else {
                            return Err(syn::Error::new_spanned(mnv.value, "Expected string literal for `custom` (e.g., `custom = \"path::to::function\"`)"));
                        }
                    }
                }
                // Handles `#[validate(range(...))]`
                Meta::List(meta_list) => {
                    if meta_list.path.is_ident("range") {
                        let mut min: Option<f64> = None;
                        let mut max: Option<f64> = None;

                        // In syn 2.0, using parse_args_with Punctuated is the correct way
                        let range_items: syn::punctuated::Punctuated<syn::MetaNameValue, syn::Token![,]> = 
                            meta_list.parse_args_with(syn::punctuated::Punctuated::parse_terminated)?;
                        
                        for kv in range_items {
                            // In syn 2.0, MetaNameValue has fields 'path' and 'value'
                            let key = kv.path;
                            let value = kv.value; // kv.value is a syn::Expr
                            if key.is_ident("min") {
                                if let Expr::Lit(ExprLit { lit: Lit::Float(lit_float), .. }) = value {
                                    min = Some(lit_float.base10_parse::<f64>()?);
                                } else {
                                    return Err(syn::Error::new_spanned(value, "`min` value for `range` must be a float literal"));
                                }
                            } else if key.is_ident("max") {
                                if let Expr::Lit(ExprLit { lit: Lit::Float(lit_float), .. }) = value {
                                    max = Some(lit_float.base10_parse::<f64>()?);
                                } else {
                                    return Err(syn::Error::new_spanned(value, "`max` value for `range` must be a float literal"));
                                }
                            }
                        }
                        if min.is_none() && max.is_none() {
                            return Err(syn::Error::new_spanned(meta_list, "`range` validation requires at least one of `min` or `max`"));
                        }
                        validations.push(Validation::Range { min: min.unwrap_or(f64::NEG_INFINITY), max: max.unwrap_or(f64::INFINITY) });
                    }
                }
            }
        }
        Ok(validations)
    }
}


/// The main entry point for the procedural macro `#[derive(ValidateCsv)]`.
#[proc_macro_derive(ValidateCsv, attributes(validate))]
pub fn validate_csv_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(f) => &f.named,
            _ => {
                return syn::Error::new_spanned(
                    &data.fields,
                    "only structs with named fields (e.g., `struct S { a: T }`) are supported"
                )
                .to_compile_error()
                .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(&input, "only structs are supported")
                .to_compile_error()
                .into();
        }
    };

    let mut field_validations = Vec::new();

    for field in fields {
        // For Fields::Named, field.ident is guaranteed to be Some(ident).
        // We clone the Ident to have an owned version for later use.
        let field_name = field.ident.as_ref().unwrap().clone(); 
        let mut validations = Vec::new();

        for attr in &field.attrs {
            // In syn 2.0, attr.path is a Path, not a method
            if attr.path().is_ident("validate") {
                // Use parse_args_with and our custom parsing function
                match attr.parse_args_with(Validation::parse_validations) {
                    Ok(mut parsed_validations) => {
                        validations.append(&mut parsed_validations);
                    }
                    Err(e) => {
                        return e.to_compile_error().into();
                    }
                }
            }
        }

        if !validations.is_empty() {
            field_validations.push(FieldValidation {
                field_name,
                validations,
            });
        }
    }

    let validation_arms = field_validations.into_iter().map(|fv| {
        let field_name_str = fv.field_name.to_string();
        // Using the owned Ident for code generation
        let field_name_ident = fv.field_name; 

        let checks = fv.validations.into_iter().map(|validation| {
            match validation {
                Validation::Required => {
                    quote! {
                        if (&self.#field_name_ident).is_none() { // More idiomatic check for Option
                            errors.push(::csv_schema_validator::ValidationError {
                                field: #field_name_str.to_string(),
                                message: "mandatory field".to_string(),
                            });
                        }
                    }
                }
                Validation::Range { min, max } => {
                    quote! {
                        let value = &self.#field_name_ident;
                        if !(#min <= *value && *value <= #max) {
                            errors.push(::csv_schema_validator::ValidationError {
                                field: #field_name_str.to_string(),
                                message: format!("value out of expected range: {} to {}", #min, #max),
                            });
                        }
                    }
                }
                Validation::Regex { regex } => {
                    quote! {
                        // --- Robust Regex Handling ---
                        // Assumes that the main crate re-exports once_cell and regex in csv_schema_validator::__private
                        use ::csv_schema_validator::__private::once_cell::sync::Lazy;
                        use ::csv_schema_validator::__private::regex;

                        // Static for the compiled regex, with a unique scope for this expansion (unique per field).
                        // Wrapped in a Result to handle compilation errors elegantly.
                        static RE: Lazy<Result<regex::Regex, regex::Error>> = Lazy::new(|| regex::Regex::new(#regex));

                        match RE.as_ref() {
                            Ok(compiled_regex) => {
                                if !compiled_regex.is_match(&self.#field_name_ident) {
                                    errors.push(::csv_schema_validator::ValidationError {
                                        field: #field_name_str.to_string(),
                                        message: "does not match the expected pattern".to_string(),
                                    });
                                }
                            }
                            Err(e) => {
                                // Reports the regex compilation error as a validation error
                                errors.push(::csv_schema_validator::ValidationError {
                                    field: #field_name_str.to_string(),
                                    message: format!("invalid regex '{}': {}", #regex, e),
                                });
                            }
                        }
                    }
                }
                Validation::Custom { path } => {
                    quote! {
                        // Calls the custom validation function defined by the user.
                        // Expected: fn(&T) -> Result<(), E> where E: Display
                        match #path(&self.#field_name_ident) {
                            Err(err) => {
                                errors.push(::csv_schema_validator::ValidationError {
                                    field: #field_name_str.to_string(),
                                    message: format!("{}", err),
                                });
                            }
                            Ok(()) => {} // Validation passed
                        }
                    }
                }
            }
        });

        // Combines checks for this field
        quote! {
            #(#checks)*
        }
    });

    let expanded = quote! {
        impl #name {
            /// Validates the instance of the struct according to the `#[validate(...)]` rules.
            /// Returns `Ok(())` if all validations pass, or `Err(Vec<ValidationError>)` listing the failures.
            pub fn validate_csv(&self) -> ::core::result::Result<(), ::std::vec::Vec<::csv_schema_validator::ValidationError>> {
                let mut errors = ::std::vec::Vec::new();
                #(#validation_arms)*
                if errors.is_empty() {
                    ::core::result::Result::Ok(())
                } else {
                    ::core::result::Result::Err(errors)
                }
            }
        }
    };

    TokenStream::from(expanded)
} 