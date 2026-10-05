//! ABI boundary repro: exercise each new stub kind through a transmuted
//! pointer exactly like the rewrite macros do (32-bit only).

#![cfg(target_arch = "x86")]
#![allow(unsafe_code)]

use lf_lb_diff::rt;

#[test]
fn abi_t2_skip() {
    let _s = rt::session();
    rt::set_roles(&[(4, rt::Role::Skip)]);
    rt::install14(1, &[], &[4], &[]);
    rt::set_skip(|a, b| {
        assert_eq!((a, b), (0x1111_1111, 0x2222_2222));
        1
    });
    let f: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(rt::callee_addr(4) as usize) };
    assert_eq!(f(0x1111_1111, 0x2222_2222), 1);
}

#[test]
fn abi_t4_write() {
    let _s = rt::session();
    rt::set_roles(&[(8, rt::Role::Write)]);
    rt::install14(1, &[], &[], &[8]);
    rt::set_write(|a, b, c, d| {
        assert_eq!((a, b, c, d), (1, 2, 3, 4));
        0
    });
    let f: extern "thiscall" fn(u32, u32, u32, u32) -> u32 =
        unsafe { core::mem::transmute(rt::callee_addr(8) as usize) };
    assert_eq!(f(1, 2, 3, 4), 0);
}

#[test]
fn abi_t1_len() {
    let _s = rt::session();
    rt::set_roles(&[(7, rt::Role::Len)]);
    rt::install14(1, &[7], &[], &[]);
    rt::set_items(vec![rt::ItemSlot {
        addr: 0x1000_0000,
        len: 8,
    }]);
    let f: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(rt::callee_addr(7) as usize) };
    assert_eq!(f(0x1000_0000), 8);
}

#[test]
fn abi_call_rewrite_failfast() {
    let _s = rt::session();
    let mut words = Box::new([0u32; 64]);
    let base = words.as_ptr().addr() as u32;
    words[0] = base + 32;
    words[8 + 11] = rt::pick_stub_addr();
    words[8 + 12] = rt::row_stub_addr();
    rt::set_roles(&[
        (3, rt::Role::Fetch),
        (4, rt::Role::Skip),
        (5, rt::Role::Classify),
        (6, rt::Role::Item),
        (7, rt::Role::Len),
        (8, rt::Role::Write),
    ]);
    rt::install14(3, &[5, 7], &[4, 6], &[8]);
    rt::set_layout(rt::FetchLayout {
        count_idx: None,
        table_a_idx: 2,
        table_b_idx: None,
    });
    rt::set_fetch(rt::FetchScript {
        ok: false,
        count: 0,
        table_a: 0,
        table_b: 0,
    });
    rt::set_pick(0);
    eprintln!("calling rewrite (fetch fails)");
    let r = lf_lb_diff::rewrites::fn_00519400::rw_00519400(
        base,
        0,
        base + 40,
        base + 48,
        base + 56,
        0x1234,
        64,
    );
    eprintln!("rewrite returned {r:#x}");
    assert_eq!(r, 0);
}

#[test]
fn abi_pick_row() {
    let _s = rt::session();
    rt::set_pick(7);
    rt::set_row_keys(vec![10, 20, 30]);
    let p: extern "thiscall" fn(u32) -> u32 =
        unsafe { core::mem::transmute(rt::pick_stub_addr() as usize) };
    assert_eq!(p(0x1234), 7);
    let r: extern "thiscall" fn(u32, u32) -> u32 =
        unsafe { core::mem::transmute(rt::row_stub_addr() as usize) };
    assert_eq!(r(0x1234, 2), 30);
}
