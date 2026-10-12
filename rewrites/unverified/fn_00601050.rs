// original: 0x00601050 copy_16_bytes

/// Copy 16 bytes from `src` to `dst`, one word at a time, and return the
/// last word copied.
///
/// The original moves the first and last words through general registers
/// and the middle two through vector registers; the observable behaviour
/// is a plain 4-word copy. Stdcall with two stack arguments: the source
/// address first, the destination second. EAX on return holds the word at
/// `[src + 0x0c]`, loaded last.
lf_checker_rt::export!(stdcall, rw_00601050(src: u32, dst: u32) -> u32 {
    unsafe {
        let w0 = (src as *const u32).read();
        let w1 = ((src + 4) as *const u32).read();
        let w2 = ((src + 8) as *const u32).read();
        let w3 = ((src + 12) as *const u32).read();
        (dst as *mut u32).write(w0);
        ((dst + 4) as *mut u32).write(w1);
        ((dst + 8) as *mut u32).write(w2);
        ((dst + 12) as *mut u32).write(w3);
        w3
    }
});
