//! Macros for poem-openapi

#![doc(html_favicon_url = "https://raw.githubusercontent.com/poem-web/poem/master/favicon.ico")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/poem-web/poem/master/logo.png")]
#![forbid(unsafe_code)]
#![deny(private_in_public, unreachable_pub)]

#[macro_use]
mod validators;

mod api;
mod common_args;
mod r#enum;
mod error;
mod multipart;
mod newtype;
mod oauth_scopes;
mod object;
mod oneof;
mod request;
mod response;
mod response_content;
mod security_scheme;
mod tags;
mod utils;
mod webhook;

use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, ItemImpl, ItemTrait};

#[proc_macro_derive(Object, attributes(oai))]
pub fn derive_object(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match object::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(Enum, attributes(oai))]
pub fn derive_enum(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match r#enum::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(OneOf, attributes(oai))]
pub fn derive_oneof(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match oneof::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(ApiResponse, attributes(oai))]
pub fn derive_response(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match response::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(ApiRequest, attributes(oai))]
pub fn derive_request(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match request::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(ResponseContent, attributes(oai))]
pub fn derive_response_content(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match response_content::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn OpenApi(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let item_impl = parse_macro_input!(input as ItemImpl);
    match api::generate(args, item_impl) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(Multipart, attributes(oai))]
pub fn derive_multipart(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match multipart::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(Tags, attributes(oai))]
pub fn derive_tags(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match tags::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(OAuthScopes, attributes(oai))]
pub fn derive_oauth_scopes(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match oauth_scopes::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(SecurityScheme, attributes(oai))]
pub fn derive_security_scheme(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match security_scheme::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn Webhook(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let item_trait = parse_macro_input!(input as ItemTrait);
    match webhook::generate(args, item_trait) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(NewType, attributes(oai))]
pub fn derive_new_type(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match newtype::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}
