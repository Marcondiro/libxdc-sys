#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    // use super::*;
    // use std::os;
    //
    // #[test]
    // fn libxdc_readme_example() {
    //     let mut trace = [0u8; 32];
    //     let mut bitmap = [0u8; 0x10000];
    //     let mut filter = [[0u64; 2]; 4];
    //     unsafe {
    //         let page_cache = page_cache_new(trace.as_ptr() as *const os::raw::c_char);
    //         let decoder = libxdc_init(
    //             filter.as_mut_ptr(),
    //             Some(page_cache_fetch),
    //             page_cache as *mut _,
    //             bitmap.as_mut_ptr() as *mut _,
    //             0x10000,
    //         );
    //         let _ret = libxdc_decode(decoder, trace.as_mut_ptr(), trace.len());
    //     }
    // }
}
