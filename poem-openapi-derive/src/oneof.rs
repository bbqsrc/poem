use darling::{
    ast::{Data, Fields},
    util::Ignored,
    FromDeriveInput, FromVariant,
};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, DeriveInput, Error, Type};

use crate::{
    common_args::ExternalDocument,
    error::GeneratorResult,
    utils::{get_crate_name, get_summary_and_description, optional_literal},
};

#[derive(FromVariant)]
#[darling(attributes(oai), forward_attrs(doc))]
struct OneOfItem {
    ident: Ident,
    fields: Fields<Type>,

    #[darling(default)]
    mapping: Option<String>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(oai), forward_attrs(doc))]
struct OneOfArgs {
    ident: Ident,
    attrs: Vec<Attribute>,
    data: Data<OneOfItem, Ignored>,

    #[darling(default)]
    internal: bool,
    property_name: String,
    #[darling(default)]
    external_docs: Option<ExternalDocument>,
}

pub(crate) fn generate(args: DeriveInput) -> GeneratorResult<TokenStream> {
    let args: OneOfArgs = OneOfArgs::from_derive_input(&args)?;
    let crate_name = get_crate_name(args.internal);
    let ident = &args.ident;
    let (title, description) = get_summary_and_description(&args.attrs)?;
    let title = optional_literal(&title);
    let description = optional_literal(&description);
    let property_name = &args.property_name;

    let e = match &args.data {
        Data::Enum(e) => e,
        _ => return Err(Error::new_spanned(ident, "OneOf can only be applied to an enum.").into()),
    };

    let mut types = Vec::new();
    let mut from_json = Vec::new();
    let mut to_json = Vec::new();
    let mut names = Vec::new();
    let mut mapping = Vec::new();

    for variant in e {
        let item_ident = &variant.ident;

        match variant.fields.len() {
            1 => {
                let object_ty = &variant.fields.fields[0];
                let mapping_name = match &variant.mapping {
                    Some(mapping) => quote!(#mapping),
                    None => {
                        quote!(::std::convert::AsRef::as_ref(&<#object_ty as #crate_name::types::Type>::name()))
                    }
                };

                types.push(object_ty);
                from_json.push(quote! {
                    ::std::option::Option::Some(property_name) if property_name == #mapping_name => {
                        <#object_ty as #crate_name::types::ParseFromJSON>::parse_from_json(value).map(Self::#item_ident).map_err(#crate_name::types::ParseError::propagate)
                    }
                });
                to_json.push(quote! {
                    Self::#item_ident(obj) => {
                        let mut value = <#object_ty as #crate_name::types::ToJSON>::to_json(obj);
                        if let ::std::option::Option::Some(obj) = value.as_object_mut() {
                            obj.insert(::std::convert::Into::into(#property_name), ::std::convert::Into::into(#mapping_name));
                        }
                        value
                    }
                });
                names.push(quote!(#mapping_name));

                if variant.mapping.is_some() {
                    mapping.push(quote! {
                        (#mapping_name, format!("#/components/schemas/{}", <#object_ty as #crate_name::types::Type>::schema_ref().unwrap_reference()))
                    });
                }
            }
            _ => {
                return Err(
                    Error::new_spanned(&variant.ident, "Incorrect oneof definition.").into(),
                )
            }
        }
    }

    let external_docs = match &args.external_docs {
        Some(external_docs) => {
            let s = external_docs.to_token_stream(&crate_name);
            quote!(::std::option::Option::Some(#s))
        }
        None => quote!(::std::option::Option::None),
    };

    let expanded = quote! {
        impl #crate_name::types::Type for #ident {
            const IS_REQUIRED: bool = true;

            type RawValueType = Self;

            type RawElementValueType = Self;

            fn name() -> ::std::borrow::Cow<'static, str> {
                ::std::convert::Into::into("object")
            }

            fn schema_ref() -> #crate_name::registry::MetaSchemaRef {
                #crate_name::registry::MetaSchemaRef::Inline(Box::new(#crate_name::registry::MetaSchema {
                    title: #title,
                    description: #description,
                    external_docs: #external_docs,
                    one_of: ::std::vec![#(<#types as #crate_name::types::Type>::schema_ref()),*],
                    properties: ::std::vec![(#property_name, #crate_name::registry::MetaSchemaRef::Inline(Box::new(#crate_name::registry::MetaSchema {
                        enum_items: ::std::vec![#(::std::convert::Into::into(#names)),*],
                        ..#crate_name::registry::MetaSchema::new("string")
                    })))],
                    discriminator: ::std::option::Option::Some(#crate_name::registry::MetaDiscriminatorObject {
                        property_name: #property_name,
                        mapping: ::std::vec![#(#mapping),*],
                    }),
                    ..#crate_name::registry::MetaSchema::new("object")
                }))
            }

            fn register(registry: &mut #crate_name::registry::Registry) {
                #(<#types as #crate_name::types::Type>::register(registry);)*
            }

            fn as_raw_value(&self) -> ::std::option::Option<&Self::RawValueType> {
                ::std::option::Option::Some(self)
            }

            fn raw_element_iter<'a>(&'a self) -> ::std::boxed::Box<dyn ::std::iter::Iterator<Item = &'a Self::RawElementValueType> + 'a> {
                ::std::boxed::Box::new(::std::iter::IntoIterator::into_iter(self.as_raw_value()))
            }
        }

        impl #crate_name::types::ParseFromJSON for #ident {
            fn parse_from_json(value: #crate_name::__private::serde_json::Value) -> ::std::result::Result<Self, #crate_name::types::ParseError<Self>> {
                match value.as_object().and_then(|obj| obj.get(#property_name)) {
                    #(#from_json,)*
                    _ => ::std::result::Result::Err(#crate_name::types::ParseError::expected_type(value)),
                }
            }
        }

        impl #crate_name::types::ToJSON for #ident {
            fn to_json(&self) -> #crate_name::__private::serde_json::Value {
                match self {
                    #(#to_json),*
                }
            }
        }
    };

    Ok(expanded)
}
