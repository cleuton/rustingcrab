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
    Custom { path: syn::Path }, 
}

impl Validation {
    /// Parses the content of `#[validate(...)]` into a vector of `Validation`.
    fn parse_validations(input: syn::parse::ParseStream) -> syn::Result<Vec<Self>> {
        let mut validations = Vec::new();

        let meta_items = syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated(input)?;

        for meta in meta_items {
            match meta {
                Meta::Path(path) => {
                    if path.is_ident("required") {
                        validations.push(Validation::Required);
                    }
                }
                Meta::NameValue(mnv) => {
                    if mnv.path.is_ident("regex") {
                        if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = mnv.value {
                            validations.push(Validation::Regex {
                                regex: lit_str.value(),
                            });
                        } else {
                            return Err(syn::Error::new_spanned(mnv.value, "Expected string literal for `regex`"));
                        }
                    } else if mnv.path.is_ident("custom") {
                        if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = mnv.value {
                            let path: syn::Path = syn::parse_str(&lit_str.value())
                                .map_err(|e| syn::Error::new_spanned(lit_str, e))?;
                            validations.push(Validation::Custom { path });
                        } else {
                            return Err(syn::Error::new_spanned(mnv.value, "Expected string literal for `custom` (e.g., `custom = \"path::to::function\"`)"));
                        }
                    }
                }
                Meta::List(meta_list) => {
                    if meta_list.path.is_ident("range") {
                        let mut min: Option<f64> = None;
                        let mut max: Option<f64> = None;

                        let range_items: syn::punctuated::Punctuated<syn::MetaNameValue, syn::Token![,]> = 
                            meta_list.parse_args_with(syn::punctuated::Punctuated::parse_terminated)?;
                        
                        for kv in range_items {
                            let key = kv.path;
                            let value = kv.value; 
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
        let field_name = field.ident.as_ref().unwrap().clone(); 
        let mut validations = Vec::new();

        for attr in &field.attrs {
            if attr.path().is_ident("validate") {
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
        let field_name_ident = fv.field_name; 

        let checks = fv.validations.into_iter().map(|validation| {
            match validation {
                Validation::Required => {
                    quote! {
                        if (&self.#field_name_ident).is_none() { 
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
                        use ::csv_schema_validator::__private::once_cell::sync::Lazy;
                        use ::csv_schema_validator::__private::regex;
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
                        match #path(&self.#field_name_ident) {
                            Err(err) => {
                                errors.push(::csv_schema_validator::ValidationError {
                                    field: #field_name_str.to_string(),
                                    message: format!("{}", err),
                                });
                            }
                            Ok(()) => {} 
                        }
                    }
                }
            }
        });

        quote! {
            #(#checks)*
        }
    });

    let expanded = quote! {
        impl #name {
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