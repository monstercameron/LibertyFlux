struct CallOut {
    eax: u32,
}

fn f(out: *const CallOut) {
    let mut b: Vec<u8> = Vec::new();
    b.extend_from_slice(&((out as usize as u32).wrapping_add(16).to_le_bytes()));
}

fn main() {}
