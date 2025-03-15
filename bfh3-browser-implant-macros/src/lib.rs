use quote::{ToTokens, quote};
use syn::{
    Block, ItemEnum, ItemTrait, TraitItem, parse_macro_input, punctuated::Punctuated, token::Comma,
};

#[proc_macro]
pub fn gen_features_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input with Block::parse_within);

    if input.len() != 2 {
        panic!("macro should contain an enum followed by a trait.");
    }

    let feature_enum = syn::parse2::<ItemEnum>(input[0].to_token_stream()).expect("parse enum");
    let feature_trait = syn::parse2::<ItemTrait>(input[1].to_token_stream()).expect("parse trait");

    let all_features = create_all_features(&feature_enum);
    let trait_impl = create_trait_impl(&feature_enum, &feature_trait);

    let bla = quote! {
        #all_features
        #feature_enum
        #feature_trait
        #trait_impl
    };

    proc_macro::TokenStream::from(bla)
}

fn create_all_features(feature_enum: &ItemEnum) -> proc_macro2::TokenStream {
    let enum_name = &feature_enum.ident;
    let feature_count = feature_enum.variants.len();

    let feature_idents = feature_enum
        .variants
        .iter()
        .map(|variant| quote! (#enum_name::#variant))
        .collect::<Punctuated<_, Comma>>();

    quote! {
        pub const ALL_FEATURES: [#enum_name; #feature_count] = [
            #feature_idents
        ];
    }
}

fn create_trait_impl(
    feature_enum: &ItemEnum,
    feature_trait: &ItemTrait,
) -> proc_macro2::TokenStream {
    let enum_ident = &feature_enum.ident;

    let trait_items = feature_trait
        .items
        .iter()
        .map(|item| {
            let TraitItem::Fn(item_fn) = item else {
                panic!("expected item fn in trait");
            };

            let sig = &item_fn.sig;
            let fn_name = &sig.ident;
            let args = sig
                .inputs
                .iter()
                .flat_map(|arg| match arg {
                    syn::FnArg::Receiver(_) => None,
                    syn::FnArg::Typed(pat_type) => Some(pat_type.pat.to_token_stream()),
                })
                .collect::<Punctuated<_, Comma>>();

            let match_branches = feature_enum
                .variants
                .iter()
                .map(|variant| {
                    let variant_ident = &variant.ident;
                    let maybe_await = sig.asyncness.map(|_| quote!(.await));
                    quote! {
                        #enum_ident::#variant_ident(x) => x.#fn_name(#args)#maybe_await
                    }
                })
                .collect::<Punctuated<_, Comma>>();

            quote! {
                pub #sig {
                    match self {
                        #match_branches
                    }
                }
            }
        })
        .collect::<proc_macro2::TokenStream>();

    quote! {
        impl #enum_ident {
            #trait_items
        }
    }
}
