# dlhookthing

Hooks `dlopen(3)` in a way that doesn't break RUNPATH support. Only works if the instruction calling dlopen is exactly five bytes long, and dlopen is not tail-called.

## How it works

Tail-calls `dlopen(3)` not to break RUNPATH support. Decrements the return address by 5 (the size of a normal call instruction) so that the hook is immediately re-called after the real `dlopen` is finished. `rax` is copied onto the stack at the beginning of the function so that the return value of the real `dlopen` can be returned from the hook.

## Notes

The C++ example must be compiled with at least `-O1` (for the tail call optimization to occur at all), `-foptimize-sibling-calls` (for the tail call, implied by -O2) and `-fno-omit-frame-pointer` (to get the address of the return address)
