// original: 0x009f52b0 word_list_report
/// Walk a word list backwards from the looked-up head, reporting each
/// nonzero word until the terminating zero word. Returns the gate answer
/// on the early path, else zero.
export!(cdecl, rw_009f52b0() -> u32 {
    unsafe {
        let gate = callee_cdecl!(1, u32,);
        if gate as u8 != 0 {
            return gate;
        }
        let head = callee_thiscall!(2, u32, relocated(0x0118_D110), 0);
        if head == 0 {
            return 0;
        }
        let mut cur = head as *const u16;
        loop {
            let w = cur.read();
            if w == 0 {
                return 0;
            }
            callee_cdecl!(3, u32, w as u32);
            cur = (cur as *const u8).sub(2) as *const u16;
        }
    }
});
