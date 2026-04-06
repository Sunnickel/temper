use quote::quote;
use std::fs;
use std::path::Path;
use syn::{parse_file, Item};

fn main() {
    println!("cargo::rerun-if-changed=src/markers.rs");
    let src_path = "src/markers.rs";

    let content = fs::read_to_string(src_path).expect("Failed to read source file");

    let ast: syn::File = parse_file(&content).expect("Failed to parse Rust source file");

    let target_module_name = "entity_types";

    let mut enum_variants = Vec::new();

    for item in ast.items {
        if let Item::Mod(module) = &item
            && module.ident == target_module_name
            && let Some((_, items)) = &module.content
        {
            for item in items {
                if let Item::Struct(struct_item) = item {
                    let ident = &struct_item.ident;
                    enum_variants.push(quote! { #ident });
                }
            }
        }
    }

    let enum_name = syn::Ident::new("EntityType", proc_macro2::Span::call_site());
    let enum_def = quote! {
        #[derive(Eq, PartialEq, serde::Serialize, serde::Deserialize, Debug, Clone, Copy, Hash,)]
        pub enum #enum_name {
            #( #enum_variants ),*
        }
    };

    let output_path = Path::new("src/entity_types.rs");
    fs::create_dir_all(output_path.parent().unwrap())
        .expect("Failed to create directory for generated enum");

    fs::write(output_path, enum_def.to_string()).expect("Failed to write generated enum to file");
}
