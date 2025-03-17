use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemFn};

pub(crate) fn basic_information(_attr: TokenStream, func: TokenStream) -> TokenStream {
    let func = parse_macro_input!(func as ItemFn);
    let func_vis = &func.vis;
    let func_block = &func.block;

    let func_decl = func.sig;
    let func_name = &func_decl.ident;
    let func_generics = &func_decl.generics;
    let func_inputs = &func_decl.inputs;
    let func_output = &func_decl.output;

    let func_decl_string = func_decl.to_token_stream().to_string();
    let func_name_string = func_name.to_string();
    let func_generics_string = func_generics.to_token_stream().to_string();
    let func_inputs_string = func_inputs.to_token_stream().to_string();
    let func_output_string = func_output.to_token_stream().to_string();

    let caller = quote! {
        #func_vis fn #func_name #func_generics(#func_inputs) #func_output {
            use std::time;
            let start = time::Instant::now();
            let res = #func_block;
            println!("aspect::basic_information: func_decl:{{{}}}, func_name:{{{}}}, func_generics:{{{}}}, func_inputs:{{{}}}, func_output:{{{}}}, time cost:{{{:?}}}",
                    #func_decl_string,
                    #func_name_string,
                    #func_generics_string,
                    #func_inputs_string,
                    #func_output_string,
                    start.elapsed());
            res
        }
    };

    caller.into()
}
