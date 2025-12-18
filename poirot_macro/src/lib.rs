use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, Fields};

#[proc_macro_derive(PoirotBuilder, attributes(builder_mandatory, builder_no_method))]
pub fn poirot_builder_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_poirot_builder(&ast)
}

fn is_option_type(ty: &syn::Type) -> bool {
    if let syn::Type::Path(type_path) = ty {
        if let Some(seg) = type_path.path.segments.last() {
            return seg.ident == "Option";
        }
    }
    false
}

fn impl_poirot_builder(ast: &syn::DeriveInput) -> TokenStream {
    let name: &syn::Ident = &ast.ident;
    let builder_name = syn::Ident::new(&format!("{}Builder", name), proc_macro2::Span::call_site());

    let data_struct = match &ast.data {
        Data::Struct(s) => s,
        _ => {
            return quote!{compile_error!("PoirotBuilder can only be derived for structs with named fields")}.into();
        }
    };

    let fields = match &data_struct.fields {
        Fields::Named(f) => &f.named,
        _ => {
            return quote!{compile_error!("PoirotBuilder can only be derived for structs with named fields")}.into();
        }
    };

    let mut mandatory_fields_decls: Vec<proc_macro2::TokenStream> = vec![];
    let mut mandatory_fields = vec![];
    let mut optional_fields_decls: Vec<proc_macro2::TokenStream> = vec![];
    let mut optional_fields: Vec<proc_macro2::TokenStream> = vec![];

    let mut with_methods = vec![];
    for field in fields {
        let ident = match &field.ident {
            Some(id) => id.clone(),
            None => continue,
        };

        let ref_ident = &ident;
        let ident_type = &field.ty;

        let decl_ident = quote! {#ref_ident: Option<#ident_type>,};

        let unwrap_safe_opt = if is_option_type(ident_type) {
            quote! {#ref_ident: self.#ref_ident.clone().unwrap_or(None),}
        } else {
            quote! {#ref_ident: self.#ref_ident.clone().unwrap_or_default(),}
        };

        let mut is_mandatory = false;
        let mut skip_method = false;
        for attr in &field.attrs {
            if attr.path().is_ident("builder_mandatory") {
                is_mandatory = true;
            } else if attr.path().is_ident("builder_no_method") {
                skip_method = true;
            }
        }

        if !skip_method {
            let method_name = syn::Ident::new(
                &format!("with_{}", ref_ident.to_string()),
                proc_macro2::Span::call_site(),
            );
            with_methods.push(quote! {
                fn #method_name(mut self, value: #ident_type) -> Self {
                    self.#ref_ident = Some(value);
                    self
                }
            });
        }

        if is_mandatory {
            mandatory_fields.push(ident);
            mandatory_fields_decls.push(decl_ident.clone());
        } else {
            optional_fields.push(unwrap_safe_opt);
            optional_fields_decls.push(decl_ident.clone());
        }
    }

    let build_checks = mandatory_fields.iter().map(|field_ident| {
        quote!{
            if self.#field_ident.is_none() {
                return Err(format!("Missing or incorrect mandatory field: {}", stringify!(#field_ident)).into());
            }
        }
    });

    quote! {

        #[derive(Debug,Default)]
        struct #builder_name {
            #(
                #mandatory_fields_decls
            )*
            // For optional fields
            #(
                #optional_fields_decls
            )*
        }

        impl #builder_name{

            #(#with_methods)*
        }

        impl #name{
            pub fn builder() -> #builder_name {
                #builder_name::default()
            }
        }

        impl PoirotBuilder for #builder_name {
            type PoirotTarget = #name;

            fn build(&self) -> Result<Self::PoirotTarget, Box<dyn std::error::Error>> {
                #(#build_checks)*

                Ok(#name {
                    #(
                        #mandatory_fields: self.#mandatory_fields.clone().unwrap(),
                    )*
                    // For optional fields, set to None if not provided
                    #(
                        #optional_fields
                    )*
                })
            }
        }
    }
    .into()
}

#[proc_macro_derive(LibraryItem, attributes(library_item))]
pub fn library_item_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_library_item(&ast)
}

fn impl_library_item(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // Optional fields used for the has_doi, has_website, has_pages fields
    let mut website_field = None::<syn::Ident>;
    let mut doi_field = None::<syn::Ident>;
    let mut pages_field = None::<syn::Ident>;
    let mut publication_year_field = None::<syn::Ident>;

    // Mandatory common fields: title, authors, source_id
    let mut title_field = Some(syn::Ident::new("title", proc_macro2::Span::call_site()));
    let mut authors_field = Some(syn::Ident::new("authors", proc_macro2::Span::call_site()));
    let mut source_id_field = Some(syn::Ident::new("source_id", proc_macro2::Span::call_site()));

    // To avoid using the same library_item twice
    let mut flag_title = false;
    let mut flag_authors = false;
    let mut flag_source_id = false;

    let data_struct = match &ast.data {
        Data::Struct(s) => s,
        _ => {
            return quote!{compile_error!("LibraryItem can only be derived for structs with named fields")}.into();
        }
    };

    let fields = match &data_struct.fields {
        Fields::Named(f) => &f.named,
        _ => {
            return quote!{compile_error!("LibraryItem can only be derived for structs with named fields")}.into();
        }
    };

    for field in fields {
        let ident = match &field.ident {
            Some(id) => id.clone(),
            None => continue,
        };

        for attr in &field.attrs {
            if !attr.path().is_ident("library_item") {
                continue;
            }

            // Parse a single identifier inside #[library_item(...)]
            let ident_in_attr = match attr.parse_args::<syn::Ident>() {
                Ok(i) => i,
                Err(_) => continue,
            };

            match ident_in_attr.to_string().as_str() {
                "website" => {
                    if website_field.is_some() {
                        return quote!{
                            compile_error!("Only one field can be annotated with #[library_item(website)]")
                        }.into();
                    }
                    website_field = Some(ident.clone());
                }
                "doi" => {
                    if doi_field.is_some() {
                        return quote!{
                            compile_error!("Only one field can be annotated with #[library_item(doi)]")
                        }.into();
                    }
                    doi_field = Some(ident.clone());
                }
                "pages" => {
                    if pages_field.is_some() {
                        return quote!{
                            compile_error!("Only one field can be annotated with #[library_item(pages)]")
                        }.into();
                    }
                    pages_field = Some(ident.clone());
                }
                "publication_year" => {
                    if publication_year_field.is_some() {
                        return quote!{
                            compile_error!("Only one field can be annotated with #[library_item(publication_year)]")
                        }.into();
                    }
                    publication_year_field = Some(ident.clone());
                }
                "title" => {
                    if flag_title {
                        return quote!{
                            compile_error!("Only one field can be annotated with #[library_item(title)]")
                        }.into();
                    }
                    title_field = Some(ident.clone());
                    flag_title = true;
                }
                "authors" => {
                    if flag_authors {
                        return quote!{
                            compile_error!("Only one field can be annotated with #[library_item(authors)]")
                        }.into();
                    }
                    authors_field = Some(ident.clone());
                    flag_authors = true;
                }
                "source_id" => {
                    if flag_source_id {
                        return quote!{
                            compile_error!("Only one field can be annotated with #[library_item(source_id)]")
                        }.into();
                    }
                    source_id_field = Some(ident.clone());
                    flag_source_id = true;
                }
                _ => (),
            }
        }
    }

    // make sure the mandatory fields exist

    if let Some(title_ident) = title_field {
        let found: bool = fields
            .iter()
            .any(|f| f.ident.as_ref().map(|i| i == &title_ident).unwrap_or(false));
        if !found {
            return quote!{
                compile_error!("LibraryItem: mandatory field title is required, it is possible to rename it using #[library_item(title)]")
            }.into();
        }
    }

    if let Some(authors_ident) = authors_field {
        let found: bool = fields.iter().any(|f| {
            f.ident
                .as_ref()
                .map(|i| i == &authors_ident)
                .unwrap_or(false)
        });
        if !found {
            return quote!{
                compile_error!("LibraryItem: mandatory field authors is required, it is possible to rename it using #[library_item(authors)]")
            }.into();
        }
    }

    if let Some(source_id_ident) = source_id_field {
        let found: bool = fields.iter().any(|f| {
            f.ident
                .as_ref()
                .map(|i| i == &source_id_ident)
                .unwrap_or(false)
        });
        if !found {
            return quote!{
                compile_error!("LibraryItem: mandatory field source_id is required, it is possible to rename it using #[library_item(source_id)]")
            }.into();
        }
    }

    let (has_doi_method, doi_eq) = if let Some(ref doi_ident) = doi_field {
        (
            quote! {
                fn has_doi(&self) -> bool {
                    self.#doi_ident.is_some()
                }
            },
            quote! {
                && if self.has_doi() {
                    self.#doi_ident == other.#doi_ident
                } else {
                    true
                }
            },
        )
    } else {
        (quote! {}, quote! {})
    };

    let (has_website_method, website_eq) = if let Some(ref website_ident) = website_field {
        (
            quote! {
                fn has_website(&self) -> bool {
                    self.#website_ident.is_some()
                }
            },
            quote! {
                && if self.has_website() {
                    self.#website_ident == other.#website_ident
                } else {
                    true
                }
            },
        )
    } else {
        (quote! {}, quote! {})
    };

    let (has_pages_method, pages_eq) = if let Some(ref pages_ident) = pages_field {
        (
            quote! {
                fn has_pages(&self) -> bool {
                    let (start, end) = (self.pages.0, self.pages.1); // or &self.pages
                    start.is_some() && end.is_some()
                }
            },
            quote! {
                && if self.has_pages() {
                    self.#pages_ident == other.#pages_ident
                } else {
                    true
                }
            },
        )
    } else {
        (quote! {}, quote! {})
    };

    quote! {
        impl LibraryItem for #name {
            #has_doi_method
            #has_website_method
            #has_pages_method
        }

        impl PartialEq for #name {
                fn eq(&self, other: &Self) -> bool{
                    self.title == other.title
                    && self.authors == other.authors
                    && self.source_id == other.source_id
                    #doi_eq
                    #website_eq
                    #pages_eq
            }
        }

        impl Eq for #name {}
    }
    .into()
}
