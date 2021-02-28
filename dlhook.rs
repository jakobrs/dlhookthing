#![feature(asm)]
#![feature(naked_functions)]

use std::os::raw::*;
use std::ffi::CStr;

extern "C" {
    #[allow(dead_code)]
    fn dlsym(a: *mut c_void, b: *const c_char) -> *mut c_void;
}

#[naked]
#[no_mangle]
#[allow(unused_variables)]
#[inline(never)]
pub extern "C" fn dlopen(file: *const c_char, mode: c_int) -> *mut c_void {
    #[no_mangle]
    static mut PHASE: u8 = 0;
    #[no_mangle]
    static     DLOPENSTR: [u8; 7] = ['d' as u8, 'l' as u8, 'o' as u8, 'p' as u8, 'e' as u8, 'n' as u8, 0];
    #[no_mangle]
    static mut ORIGDLOPEN: *mut c_void = 0 as *mut c_void;
    #[no_mangle]
    static mut OLDFILE: *const c_char = 0 as *const c_char;
    #[no_mangle]
    static mut OLDMODE: c_int = 0;

    unsafe {
        asm!(
            "push rbp",
            "mov rbp, rsp",
            "sub rsp, 0x20",
            "mov [rbp - 0x8], rax",
            "mov [rbp - 0x10], rdi",
            "mov [rbp - 0x14], esi",

            "mov rax, [rip + PHASE@GOTPCREL]",
            "mov rax, [rax]",
            "cmp rax, 0",
            "jne 2f",

            // PHASE == 0
            "mov rcx, [rip + ORIGDLOPEN@GOTPCREL]",
            "mov rcx, [rcx]",
            "cmp rcx, 0",
            "jne 1f",

            // ORIGDLOPEN == 0
            "mov rdi, -1",
            "mov rsi, [rip + DLOPENSTR@GOTPCREL]",
            "call [rip + dlsym@GOTPCREL]",
            "mov rcx, [rip + ORIGDLOPEN@GOTPCREL]",
            "mov [rcx], rax",
            "mov rcx, rax",

            // ORIGDLOPEN != 0 (hopefully)
            "1:",
            "mov rdx, [rip + PHASE@GOTPCREL]",
            "mov byte ptr [rdx], 1",
            "sub qword ptr [rbp + 8], 5",

            "mov rdi, [rbp - 0x10]",
            "mov esi, [rbp - 0x14]",

            //"int 3",
            "mov rax, [rip + OLDFILE@GOTPCREL]",
            "mov [rax], rdi",
            "mov rax, [rip + OLDMODE@GOTPCREL]",
            "mov [rax], esi",

            "mov rsp, rbp",
            "pop rbp",
            "jmp rcx",

            // PHASE == 1
            "2:",
            //"int 3",
            "mov rdx, [rip + OLDFILE@GOTPCREL]",
            "mov rdi, [rdx]",
            "mov rdx, [rip + OLDMODE@GOTPCREL]",
            "mov esi, [rdx]",
            "mov rdx, [rbp - 0x8]",
            "call [rip + posthook@GOTPCREL]",

            "mov rcx, [rip + PHASE@GOTPCREL]",
            "mov byte ptr [rcx], 0",
            "mov rax, [rbp - 0x8]",
            "mov rsp, rbp",
            "pop rbp",
            "ret",
            options(noreturn)
        );
    }
}

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn posthook(file: *const c_char, mode: c_int, rax: *mut c_void) {
    println!("Successfully opened {:?} with mode {}, returning {:?}", CStr::from_ptr(file), mode, rax);
}
