#![feature(asm)]
#![feature(naked_functions)]

use std::os::raw::*;

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

    unsafe {
        asm!(
            "push rbp",
            "mov rbp, rsp",
            "sub rsp, 32",
            "mov [rbp - 8], rax",
            "mov [rbp - 16], rdi",
            "mov [rbp - 20], esi",

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

            "mov rdi, [rbp - 16]",
            "mov esi, [rbp - 20]",
            "mov rsp, rbp",
            "pop rbp",
            "jmp rcx",

            // PHASE == 1
            "2:",
            "mov rcx, [rip + PHASE@GOTPCREL]",
            "mov byte ptr [rcx], 0",
            "mov rax, [rbp - 8]",
            "mov rsp, rbp",
            "pop rbp",
            "ret",
            options(noreturn)
        );
    }
}
