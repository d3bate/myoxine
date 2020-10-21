pub fn token_streams_are_equal(
    stream1: proc_macro2::TokenStream,
    stream2: proc_macro2::TokenStream,
) -> bool {
    stream1
        .into_iter()
        .zip(stream2)
        .filter(|(token1, token2)| match token1 {
            proc_macro2::TokenTree::Group(group1) => {
                if let proc_macro2::TokenTree::Group(group2) = token2 {
                    let delimiter1 = group1.delimiter();
                    let delimiter2 = group2.delimiter();
                    delimiter1 != delimiter2
                        || !token_streams_are_equal(group1.stream(), group2.stream())
                } else {
                    true
                }
            }
            proc_macro2::TokenTree::Ident(ident1) => {
                if let proc_macro2::TokenTree::Ident(ident2) = token2 {
                    ident1 != ident2
                } else {
                    true
                }
            }
            proc_macro2::TokenTree::Punct(punct1) => {
                if let proc_macro2::TokenTree::Punct(punct2) = token2 {
                    punct1.to_string() != punct1.to_string()
                } else {
                    true
                }
            }
            proc_macro2::TokenTree::Literal(lit1) => false,
        })
        .next()
        .is_none()
}
