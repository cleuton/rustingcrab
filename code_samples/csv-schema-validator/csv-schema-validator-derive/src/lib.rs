// csv-schema-validator-derive/src/lib.rs
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Expr, ExprLit, Fields, GenericArgument, Ident, Lit, Meta,
    PathArguments, Type,
};

// Armazena validações por campo
struct FieldValidation {
    field_name: Ident,
    is_option: bool, // [FIX] passamos a carregar se o campo é Option<T>
    validations: Vec<Validation>,
}

// Tipos de validações suportadas
enum Validation {
    Range { min: f64, max: f64 },
    Regex { regex: String },
    Required,
    Custom { path: syn::Path },
    Length { min: usize, max: usize },
}

impl Validation {
    /// Faz o parse de #[validate(...)] em uma lista de Validation
    fn parse_validations(input: syn::parse::ParseStream) -> syn::Result<Vec<Self>> {
        let mut validations = Vec::new();
        let meta_items =
            syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated(input)?;

        for meta in meta_items {
            match meta {
                Meta::Path(path) => {
                    if path.is_ident("required") {
                        validations.push(Validation::Required);
                    }
                }
                Meta::NameValue(mnv) => {
                    if mnv.path.is_ident("regex") {
                        if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = mnv.value {
                            validations.push(Validation::Regex { regex: s.value() });
                        } else {
                            return Err(syn::Error::new_spanned(
                                mnv.value,
                                "Expected string literal for `regex`",
                            ));
                        }
                    } else if mnv.path.is_ident("custom") {
                        if let Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) = mnv.value {
                            let path: syn::Path =
                                syn::parse_str(&s.value()).map_err(|e| syn::Error::new_spanned(s, e))?;
                            validations.push(Validation::Custom { path });
                        } else {
                            return Err(syn::Error::new_spanned(
                                mnv.value,
                                "Expected string literal for `custom` (e.g., custom = \"path::to::fn\")",
                            ));
                        }
                    }
                }
                Meta::List(meta_list) => {
                    if meta_list.path.is_ident("length") {
                        let mut min: Option<usize> = None;
                        let mut max: Option<usize> = None;

                        let items: syn::punctuated::Punctuated<syn::MetaNameValue, syn::Token![,]> =
                            meta_list.parse_args_with(
                                syn::punctuated::Punctuated::parse_terminated,
                            )?;

                        for kv in items {
                            if kv.path.is_ident("min") {
                                if let Expr::Lit(ExprLit { lit: Lit::Int(i), .. }) = kv.value {
                                    min = Some(i.base10_parse::<usize>()?);
                                } else {
                                    return Err(syn::Error::new_spanned(
                                        kv.value,
                                        "`min` for `length` must be an integer literal",
                                    ));
                                }
                            } else if kv.path.is_ident("max") {
                                if let Expr::Lit(ExprLit { lit: Lit::Int(i), .. }) = kv.value {
                                    max = Some(i.base10_parse::<usize>()?);
                                } else {
                                    return Err(syn::Error::new_spanned(
                                        kv.value,
                                        "`max` for `length` must be an integer literal",
                                    ));
                                }
                            }
                        }

                        if min.is_none() && max.is_none() {
                            return Err(syn::Error::new_spanned(
                                meta_list,
                                "`length` requires at least one of `min` or `max`",
                            ));
                        }
                        if let Some(mx) = max {
                            if mx == 0 {
                                return Err(syn::Error::new_spanned(
                                    meta_list,
                                    "`max` for `length` cannot be zero",
                                ));
                            }
                        }
                        if let (Some(a), Some(b)) = (min, max) {
                            if a > b {
                                return Err(syn::Error::new_spanned(
                                    meta_list,
                                    "`min` must be <= `max` for `length`",
                                ));
                            }
                        }

                        validations.push(Validation::Length {
                            min: min.unwrap_or(0),
                            max: max.unwrap_or(usize::MAX),
                        });
                    } else if meta_list.path.is_ident("range") {
                        let mut min: Option<f64> = None;
                        let mut max: Option<f64> = None;

                        let items: syn::punctuated::Punctuated<syn::MetaNameValue, syn::Token![,]> =
                            meta_list.parse_args_with(
                                syn::punctuated::Punctuated::parse_terminated,
                            )?;

                        for kv in items {
                            if kv.path.is_ident("min") {
                                if let Expr::Lit(ExprLit { lit: Lit::Float(f), .. }) = kv.value {
                                    min = Some(f.base10_parse::<f64>()?);
                                } else {
                                    return Err(syn::Error::new_spanned(
                                        kv.value,
                                        "`min` for `range` must be a float literal",
                                    ));
                                }
                            } else if kv.path.is_ident("max") {
                                if let Expr::Lit(ExprLit { lit: Lit::Float(f), .. }) = kv.value {
                                    max = Some(f.base10_parse::<f64>()?);
                                } else {
                                    return Err(syn::Error::new_spanned(
                                        kv.value,
                                        "`max` for `range` must be a float literal",
                                    ));
                                }
                            }
                        }

                        if min.is_none() && max.is_none() {
                            return Err(syn::Error::new_spanned(
                                meta_list,
                                "`range` requires at least one of `min` or `max`",
                            ));
                        }

                        validations.push(Validation::Range {
                            min: min.unwrap_or(f64::NEG_INFINITY),
                            max: max.unwrap_or(f64::INFINITY),
                        });
                    }
                }
            }
        }

        Ok(validations)
    }
}

// [FIX] helper para detectar Option<T>
fn option_inner_type(ty: &Type) -> Option<&Type> {
    if let Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.last() {
            if seg.ident == "Option" {
                if let PathArguments::AngleBracketed(args) = &seg.arguments {
                    if let Some(GenericArgument::Type(inner_ty)) = args.args.first() {
                        return Some(inner_ty);
                    }
                }
            }
        }
    }
    None
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
                    "only structs with named fields are supported",
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
        let is_option = option_inner_type(&field.ty).is_some(); // [FIX] capturamos se é Option<T>
        let mut validations = Vec::new();

        for attr in &field.attrs {
            if attr.path().is_ident("validate") {
                match attr.parse_args_with(Validation::parse_validations) {
                    Ok(mut v) => validations.append(&mut v),
                    Err(e) => return e.to_compile_error().into(),
                }
            }
        }

        if !validations.is_empty() {
            field_validations.push(FieldValidation {
                field_name,
                is_option,
                validations,
            });
        }
    }

    let validation_arms = field_validations.into_iter().map(|fv| {
        let field_name_str = fv.field_name.to_string();
        let field_name_ident = fv.field_name;
        let fv_is_option = fv.is_option;

        let checks = fv.validations.into_iter().map(|validation| match validation {
            Validation::Required => {
                // mantém comportamento: se None => erro
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
                // [FIX] valida range apenas quando Some(v) em Option<f64>
                if fv_is_option {
                    quote! {
                        if let Some(value) = &self.#field_name_ident {
                            if !(#min <= *value && *value <= #max) {
                                errors.push(::csv_schema_validator::ValidationError {
                                    field: #field_name_str.to_string(),
                                    message: format!("value out of expected range: {} to {}", #min, #max),
                                });
                            }
                        }
                    }
                } else {
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
            }
            Validation::Length { min, max } => {
                // [FIX] trata Option<String>: valida apenas quando Some(s)
                if fv_is_option {
                    quote! {
                        if let Some(value) = &self.#field_name_ident {
                            let len = value.len();
                            if len < #min || len > #max {
                                errors.push(::csv_schema_validator::ValidationError {
                                    field: #field_name_str.to_string(),
                                    message: format!("length out of expected range: {} to {}", #min, #max),
                                });
                            }
                        }
                    }
                } else {
                    quote! {
                        let value = &self.#field_name_ident;
                        let len = value.len();
                        if len < #min || len > #max {
                            errors.push(::csv_schema_validator::ValidationError {
                                field: #field_name_str.to_string(),
                                message: format!("length out of expected range: {} to {}", #min, #max),
                            });
                        }
                    }
                }
            }
            Validation::Regex { regex } => {
                // [FIX] reutiliza corpo mas injeta binding 'value' diferente para Option<String>
                let regex_body = quote! {
                    use ::csv_schema_validator::__private::once_cell::sync::Lazy;
                    use ::csv_schema_validator::__private::regex;
                    static RE: Lazy<Result<regex::Regex, regex::Error>> = Lazy::new(|| regex::Regex::new(#regex));

                    match RE.as_ref() {
                        Ok(compiled_regex) => {
                            if !compiled_regex.is_match(value) {
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
                };

                if fv_is_option {
                    quote! {
                        if let Some(value) = &self.#field_name_ident {
                            #regex_body
                        }
                    }
                } else {
                    quote! {
                        let value = &self.#field_name_ident;
                        #regex_body
                    }
                }
            }
            Validation::Custom { path } => {
                // [FIX] chama função apenas quando Some(v) para Option<T>
                if fv_is_option {
                    quote! {
                        if let Some(value) = &self.#field_name_ident {
                            match #path(value) {
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
                } else {
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
                    Ok(())
                } else {
                    Err(errors)
                }
            }
        }
    };

    TokenStream::from(expanded)
}
