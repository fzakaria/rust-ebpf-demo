use core::mem::transmute;
use std::ffi::{CString};
use std::os::raw::{c_int, c_char};

///
/// typically there exists a coercsion from the bpf_func_id to a function pointer.
/// BPF_FUNC_trace_printk has the value 6
/// https://elixir.bootlin.com/linux/v4.7/source/include/uapi/linux/bpf.h#L153
/// https://gist.github.com/alessandrod/293bb4dfdf82fd8cc5f7de98e01b10d6
/// 
/// ```c
/// static int (*bpf_trace_printk)(const char *fmt, int fmt_size, ...) =
/// (void *) BPF_FUNC_trace_printk;
/// ```
/// We must always inline this because eBPF does not support function calls
/// (aside from the helpers).
///
#[inline(always)]
fn bpf_trace_printk_helper(msg: &str) -> i32 {
    // Unfortunately, at the moment this does not support
    // variadic printk
    let printk : fn(*const c_char, c_int) -> c_int = unsafe {
        transmute::<>(6 as i64)
    };

    let c_str = CString::new(msg).unwrap();
    return printk(c_str.as_ptr(), c_str.to_bytes().len() as i32);
}

#[link_section = "license"]
/// str/String in rust are not null terminated, we have to put one in manually
pub const LICENSE: *const c_char = "my string\0".as_ptr() as *const c_char;


#[no_mangle]
#[link_section = "hello_world"]
// LLVM has a bug where the section name cannot be same as function name
// LLVM ERROR: 'hello_world' label emitted multiple times to assembly file
// https://patchwork.ozlabs.org/patch/808209/
pub extern "C" fn hello_world_filter(_ctx: *mut u8) -> i32 {
    bpf_trace_printk_helper("hello world!\n");
    return 0;
}