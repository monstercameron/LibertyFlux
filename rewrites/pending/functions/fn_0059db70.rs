// original: 0x0059db70 init_state_block
// Zero-initialize a 0x3C-byte state block, then atomically publish it.
//
// All multi-byte fields are zeroed individually (several bytes at 0x11,
// 0x13-0x14, 0x16-0x17 and the 0x19-0x27 gaps are left untouched), the tail
// two words are zeroed, the block tail is handed to InterlockedExchange
// with value 0 (a no-op exchange that returns the previous 0), the second
// tail word is zeroed again, and `this` is returned.
export!(thiscall, rw_0059DB70(this: *mut u8) -> u32 {
    unsafe {
        *(this as *mut u32) = 0;
        *((this.add(4)) as *mut u32) = 0;
        *((this.add(8)) as *mut u32) = 0;
        *((this.add(0xC)) as *mut u32) = 0;
        *((this.add(0x10)) as *mut u16) = 0;
        *this.add(0x12) = 0;
        *this.add(0x15) = 0;
        *((this.add(0x18)) as *mut u32) = 0;
        *this.add(0x1C) = 0;
        *((this.add(0x20)) as *mut u32) = 0;
        *this.add(0x24) = 0;
        *((this.add(0x28)) as *mut u32) = 0;
        *((this.add(0x2C)) as *mut u32) = 0;
        *((this.add(0x30)) as *mut u32) = 0;
        let tail = this.add(0x34);
        *(tail as *mut u32) = 0;
        *((tail.add(4)) as *mut u32) = 0;
        callee_stdcall!(1, u32, tail as u32, 0);
        *((tail.add(4)) as *mut u32) = 0;
        this as u32
    }
});
