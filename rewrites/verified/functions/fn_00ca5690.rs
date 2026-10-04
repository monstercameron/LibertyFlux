// original: 0x00ca5690 event_id_dispatcher

use lf_k2_rt::{callee_cdecl, callee_thiscall, export, global};

#[inline(always)]
unsafe fn load_u32(addr: u32) -> u32 {
    *(addr as *const u32)
}

#[inline(always)]
unsafe fn store_u32(addr: u32, v: u32) {
    *(addr as *mut u32) = v;
}

#[inline(always)]
unsafe fn vcall8(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(8));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall12(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(12));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall16(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(16));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall20(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(20));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall24(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(24));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall28(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(28));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall36(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(36));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall40(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(40));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall44(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(44));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall48(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(48));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall52(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(52));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall56(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(56));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall64(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(64));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall68(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(68));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall72(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(72));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall76(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(76));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall80(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(80));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall84(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(84));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall88(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(88));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall92(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(92));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall96(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(96));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall100(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(100));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall104(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(104));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall108(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(108));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall112(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(112));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall116(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(116));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall120(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(120));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall124(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(124));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall128(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(128));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall132(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(132));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall136(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(136));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall140(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(140));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall144(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(144));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall148(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(148));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall152(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(152));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall156(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(156));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall160(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(160));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall164(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(164));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall168(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(168));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall172(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(172));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall176(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(176));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall180(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(180));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall184(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(184));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall188(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(188));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall192(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(192));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall196(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(196));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall200(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(200));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall204(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(204));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall208(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(208));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall216(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(216));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall220(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(220));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall224(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(224));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall228(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(228));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall232(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(232));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall236(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(236));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall240(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(240));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall244(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(244));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall248(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(248));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall252(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(252));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall256(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(256));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall260(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(260));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall264(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(264));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall268(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(268));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall272(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(272));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall276(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(276));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall280(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(280));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall284(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(284));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall288(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(288));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall292(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(292));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}
#[inline(always)]
unsafe fn vcall296(this: u32, a: u32, b: u32, c: u32) {
    unsafe {
        let vtable = load_u32(this);
        let target = load_u32(vtable.wrapping_add(296));
        let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
            core::mem::transmute(target as usize);
        f(this, a, b, c);
    }
}

#[inline(never)]
unsafe fn special_flag_or_factory(this: u32, _a0: u32) {
    unsafe {
        if (*(load_u32(this.wrapping_add(4)).wrapping_add(0x26c) as *const u8) & 1) != 0 {
            return;
        }
        let sel = if (callee_cdecl!(72, u32,) & 0xff) == 0 { 0x201 } else { 0x200 };
        let f = callee_thiscall!(73, u32, *global::<u32>(0x167e2a0),);
        if f == 0 {
            store_u32(this.wrapping_add(0xc), 0);
        } else {
            store_u32(this.wrapping_add(0xc), callee_thiscall!(74, u32, f, sel, 0));
        }
    }
}

#[inline(never)]
unsafe fn special_factory_b(this: u32, _a0: u32) {
    unsafe {
        let f = callee_thiscall!(75, u32, *global::<u32>(0x167e2a0),);
        if f == 0 {
            store_u32(this.wrapping_add(0xc), 0);
        } else {
            store_u32(this.wrapping_add(0xc), callee_thiscall!(76, u32, f,));
        }
    }
}

#[inline(never)]
unsafe fn special_factory_c(this: u32, a0: u32) {
    unsafe {
        let f = callee_thiscall!(77, u32, *global::<u32>(0x167e2a0),);
        if f == 0 {
            store_u32(this.wrapping_add(0xc), 0);
            return;
        }
        let sub = load_u32(a0.wrapping_add(0x10));
        let sev: u32 = if sub == 0 {
            0
        } else {
            let stable = load_u32(sub);
            let starget = load_u32(stable.wrapping_add(4));
            let sev_of: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(starget as usize);
            sev_of(sub)
        };
        store_u32(
            this.wrapping_add(0xc),
            callee_thiscall!(79, u32, f, 0x3e8, load_u32(a0.wrapping_add(0xc)), sev, 0),
        );
    }
}

export!(thiscall, rw_b08_f3(this: u32, a0: u32) -> u32 {
    unsafe {
        store_u32(this.wrapping_add(8), 0);
        store_u32(this.wrapping_add(0xc), 0);
        store_u32(this.wrapping_add(0x10), 0);
        store_u32(this.wrapping_add(0x14), 0);
        store_u32(this.wrapping_add(0x18), 0);
        store_u32(this.wrapping_add(0x3c), 0);
        let table = load_u32(load_u32(this.wrapping_add(4)).wrapping_add(0x224));
        let mut first: u32 = 0;
        let mut i: u32 = 0;
        while i < 5 {
            let v = load_u32(table.wrapping_add(0x44).wrapping_add(i.wrapping_mul(4)));
            if v != 0 {
                first = v;
                break;
            }
            i += 1;
        }
        let (ctx, tail): (u32, u32) = if first == 0 {
            (0, 0)
        } else {
            let mut t = first;
            while load_u32(t.wrapping_add(8)) != 0 {
                t = load_u32(t.wrapping_add(8));
            }
            (first, t)
        };
        let etable = load_u32(a0);
        let etarget = load_u32(etable.wrapping_add(4));
        let event_id_of: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(etarget as usize);
        let v = event_id_of(a0).wrapping_sub(1);
        match v {
        0x00 => vcall40(this, a0, ctx, tail),
        0x01 => vcall12(this, a0, ctx, tail),
        0x02 => vcall16(this, a0, ctx, tail),
        0x03 => vcall44(this, a0, ctx, tail),
        0x04 => vcall52(this, a0, ctx, tail),
        0x05 => vcall48(this, a0, ctx, tail),
        0x06 => vcall36(this, a0, ctx, tail),
        0x08 => vcall24(this, a0, ctx, tail),
        0x09 => vcall56(this, a0, ctx, tail),
        0x0a => {}
        0x0b => vcall20(this, a0, ctx, tail),
        0x0c => vcall104(this, a0, ctx, tail),
        0x0d => vcall68(this, a0, ctx, tail),
        0x0e => vcall76(this, a0, ctx, tail),
        0x10 => vcall80(this, a0, ctx, tail),
        0x11 => vcall84(this, a0, ctx, tail),
        0x12 => vcall88(this, a0, ctx, tail),
        0x18 => vcall92(this, a0, ctx, tail),
        0x19 => vcall96(this, a0, ctx, tail),
        0x1a => vcall100(this, a0, ctx, tail),
        0x1b => vcall112(this, a0, ctx, tail),
        0x1e => vcall28(this, a0, ctx, tail),
        0x1f => vcall116(this, a0, ctx, tail),
        0x20 => { special_flag_or_factory(this, a0) }
        0x23..=0x24 => vcall128(this, a0, ctx, tail),
        0x25..=0x26 => vcall124(this, a0, ctx, tail),
        0x28 => vcall184(this, a0, ctx, tail),
        0x29 => vcall188(this, a0, ctx, tail),
        0x2a => vcall192(this, a0, ctx, tail),
        0x2b => vcall196(this, a0, ctx, tail),
        0x2c => vcall200(this, a0, ctx, tail),
        0x2d => vcall204(this, a0, ctx, tail),
        0x2f => vcall100(this, a0, ctx, tail),
        0x30 => vcall208(this, a0, ctx, tail),
        0x31 => vcall156(this, a0, ctx, tail),
        0x32 => vcall160(this, a0, ctx, tail),
        0x33 => vcall152(this, a0, ctx, tail),
        0x34 => vcall148(this, a0, ctx, tail),
        0x35 => vcall260(this, a0, ctx, tail),
        0x36 => vcall264(this, a0, ctx, tail),
        0x37 => vcall64(this, a0, ctx, tail),
        0x39 => vcall164(this, a0, ctx, tail),
        0x3a => vcall220(this, a0, ctx, tail),
        0x3b => vcall168(this, a0, ctx, tail),
        0x3c => vcall144(this, a0, ctx, tail),
        0x3d => vcall140(this, a0, ctx, tail),
        0x3e => vcall136(this, a0, ctx, tail),
        0x40 => {}
        0x41 => vcall172(this, a0, ctx, tail),
        0x42 => vcall216(this, a0, ctx, tail),
        0x45 => vcall176(this, a0, ctx, tail),
        0x48 => vcall184(this, a0, ctx, tail),
        0x4a => vcall224(this, a0, ctx, tail),
        0x4d => vcall228(this, a0, ctx, tail),
        0x4e => vcall180(this, a0, ctx, tail),
        0x52 => vcall232(this, a0, ctx, tail),
        0x5c => { special_factory_b(this, a0) }
        0x5f => vcall128(this, a0, ctx, tail),
        0x60 => vcall140(this, a0, ctx, tail),
        0x61 => vcall236(this, a0, ctx, tail),
        0x65 => {}
        0x66 => vcall240(this, a0, ctx, tail),
        0x73 => vcall208(this, a0, ctx, tail),
        0x74 => vcall132(this, a0, ctx, tail),
        0x76 => vcall244(this, a0, ctx, tail),
        0x78 => vcall248(this, a0, ctx, tail),
        0x7a => vcall108(this, a0, ctx, tail),
        0x7b => vcall120(this, a0, ctx, tail),
        0x7c => vcall72(this, a0, ctx, tail),
        0x7f => vcall252(this, a0, ctx, tail),
        0x80 => vcall256(this, a0, ctx, tail),
        0x81 => { special_factory_c(this, a0) }
        0x83 => vcall288(this, a0, ctx, tail),
        0x85 => vcall292(this, a0, ctx, tail),
        0x86 => vcall268(this, a0, ctx, tail),
        0x87 => vcall272(this, a0, ctx, tail),
        0x88 => vcall276(this, a0, ctx, tail),
        0x89 => vcall280(this, a0, ctx, tail),
        0x8a => vcall296(this, a0, ctx, tail),
        0x8b => vcall284(this, a0, ctx, tail),
            _ => vcall8(this, a0, ctx, tail),
        }
        let poll = callee_thiscall!(80, u32, this.wrapping_add(0x34),);
        if load_u32(this.wrapping_add(0xc)) == 0
            && load_u32(this.wrapping_add(8)) == 0
            && load_u32(this.wrapping_add(0x10)) == 0
            && load_u32(this.wrapping_add(0x14)) == 0
            && load_u32(this.wrapping_add(0x18)) == 0
            && poll == 0
        {
            callee_thiscall!(81, u32, this.wrapping_add(0x34), a0);
        }
        0
    }
});
