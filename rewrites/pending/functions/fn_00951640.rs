// original: 0x00951640 object_create_and_register
/// Create a derived object through a virtual factory and register it.
///
/// Takes an object pointer. A scripted gate helper checks the word at
/// +0x14 against a global flag: a zero answer runs a scripted deny
/// helper and returns 0. Otherwise a scripted prepare helper runs with
/// a zeroed three-word scratch buffer, then a virtual factory call is
/// made on a global object: slot +0x1c when the byte at +9 has bit 4
/// set (passing a [0x100, 0] descriptor plus the word value, a zero and
/// the scratch buffer), slot +0x14 otherwise (passing a zeroed
/// descriptor plus the word value and the scratch buffer). The slot-A
/// answer goes through a scripted all-zero three-word submit, gets byte 0x64 at
/// +0x371, is stored to a global slot, and goes through a scripted
/// post helper with a zeroed two-word buffer. A null factory answer
/// returns 0; otherwise the object registers through a scripted triple
/// (1, the dword at +0xc, the object), stores that dword at +0x64 and
/// a scripted kind answer at +0x68, normalises bits 10-14 of the dword
/// at +0x28 to 0x1400 unless bits 10-14 already read 0xc00, runs a
/// scripted action with parameter 3 and two scripted hooks, sets byte
/// 2 at +0x41, and returns the object.
export!(cdecl, rw_00951640(obj: u32) -> u32 {
    unsafe {
        const G_FLAG: u32 = 0x12B4138;
        const G_FACTORY: u32 = 0x166D9F4;
        const G_SLOT: u32 = 0x11F70FC;
        const ID_GATE: u32 = 0;
        const ID_DENY: u32 = 1;
        const ID_PREP: u32 = 2;
        const ID_VA: u32 = 3;
        const ID_VB: u32 = 4;
        const ID_SUBMIT: u32 = 5;
        const ID_POST: u32 = 6;
        const ID_REG: u32 = 7;
        const ID_KIND: u32 = 8;
        const ID_ACT: u32 = 9;
        const ID_PRE: u32 = 10;
        const ID_FIN: u32 = 11;
        const VTBL_SLOT_A: u32 = 0x1C;
        const VTBL_SLOT_B: u32 = 0x14;

        let flag = *(global::<u32>(G_FLAG));
        let wv = core::ptr::read_unaligned((obj.wrapping_add(0x14)) as *const u16) as u32;
        // The gate answers in AL only; the upper bytes are stub residue.
        let gate_full: u32 = callee_cdecl!(ID_GATE, u32, wv, flag);
        if gate_full & 0xFF == 0 {
            let _: u32 = callee_cdecl!(ID_DENY, u32, wv, flag, 8);
            return 0;
        }
        let buf_out = [0u32; 3];
        let pob = buf_out.as_ptr() as u32;
        let _: u32 = callee_thiscall!(ID_PREP, u32, obj, pob);
        let vobj = *(global::<u32>(G_FACTORY));
        let vtbl = *(vobj as *const u32);
        let mode = *((obj.wrapping_add(9)) as *const u8);
        let created: u32;
        if mode & 0x10 != 0 {
            let buf_a = [0x100u32, 0u32];
            let pa = buf_a.as_ptr() as u32;
            let fa = *((vtbl.wrapping_add(VTBL_SLOT_A)) as *const u32);
            let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(fa as usize);
            let ans = f(vobj, pa, wv, 0, pob, 0);
            let _: u32 = callee_thiscall!(ID_SUBMIT, u32, ans, 0, 0, 0);
            *((ans.wrapping_add(0x371)) as *mut u8) = 0x64;
            *(global::<u32>(G_SLOT)) = ans;
            let buf2 = [0u32; 2];
            let p2 = buf2.as_ptr() as u32;
            let _: u32 = callee_cdecl!(ID_POST, u32, ans, p2);
            created = ans;
        } else {
            let buf_b = [0u32; 2];
            let pb = buf_b.as_ptr() as u32;
            let fb = *((vtbl.wrapping_add(VTBL_SLOT_B)) as *const u32);
            let f: extern "thiscall" fn(u32, u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(fb as usize);
            created = f(vobj, pb, wv, pob, 0, 0);
        }
        if created == 0 {
            return 0;
        }
        let field_c = *((obj.wrapping_add(0xC)) as *const u32);
        let _: u32 = callee_cdecl!(ID_REG, u32, 1, field_c, created);
        *((created.wrapping_add(0x64)) as *mut u32) = field_c;
        let kind: u32 = callee_cdecl!(ID_KIND, u32, field_c);
        let mut bits = *((created.wrapping_add(0x28)) as *const u32);
        *((created.wrapping_add(0x68)) as *mut u32) = kind;
        if bits & 0x7C00 != 0x0C00 {
            bits = (bits & 0xFFFF97FF) | 0x1400;
            *((created.wrapping_add(0x28)) as *mut u32) = bits;
        }
        let _: u32 = callee_thiscall!(ID_ACT, u32, created, 3);
        let _: u32 = callee_cdecl!(ID_PRE, u32, created, 0);
        *((created.wrapping_add(0x41)) as *mut u8) = 2;
        let _: u32 = callee_cdecl!(ID_FIN, u32, created, 0);
        created
    }
});
