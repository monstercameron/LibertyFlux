//! Host tests: descriptor sanity and the one-function lift.

use lf_asset_slots::{AssetDesc, AssetRegistry, Slot, desc::DESCS, register_slot};

struct AR {
    seen: Vec<u32>,
}

impl AssetRegistry for AR {
    fn register_slot(&mut self, asset: &AssetDesc) -> Slot {
        self.seen.push(asset.index);
        Slot::new(asset.index ^ 0xABCD)
    }
}

#[test]
fn asset_registers_by_index() {
    assert_eq!(DESCS.len(), 80);
    for (i, d) in DESCS.iter().enumerate() {
        assert!(!d.name.is_empty());
        assert_eq!(d.index as usize, i);
    }
    let mut reg = AR { seen: Vec::new() };
    assert_eq!(register_slot(&mut reg, &DESCS[3]).get(), 3 ^ 0xABCD);
    assert_eq!(reg.seen, vec![3]);
}
