//! Portable lift of the per-item visibility and layout commit.

use super::container::MontageContainer;
use super::item_world::{GeometryObject, GeometryValue, ItemGlobals, ItemTag, ItemWorld};

const HALF: f32 = f32::from_bits(0x3F00_0000);
const ONE: f32 = f32::from_bits(0x3F80_0000);

pub(super) fn mul(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) * core::hint::black_box(b)
}

pub(super) fn sub(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) - core::hint::black_box(b)
}

pub(super) fn add(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) + core::hint::black_box(b)
}

pub(super) fn div(a: f32, b: f32) -> f32 {
    core::hint::black_box(a) / core::hint::black_box(b)
}

pub(super) fn viewport_edge(value: i32, scale_bits: u32) -> f32 {
    let raw = mul(value as f32, f32::from_bits(scale_bits));
    if 0.0 > raw {
        0.0
    } else if raw > ONE {
        ONE
    } else {
        raw
    }
}

pub(super) fn merge_max(own_edge: f32, source_edge: f32) -> f32 {
    if own_edge > source_edge {
        own_edge
    } else {
        source_edge
    }
}

pub(super) fn merge_min(source_edge: f32, own_edge: f32) -> f32 {
    if source_edge > own_edge {
        own_edge
    } else {
        source_edge
    }
}

pub(super) fn viewport_contains(
    width: f32,
    left: f32,
    right: f32,
    height: f32,
    top: f32,
    bottom: f32,
) -> bool {
    (width > left) && (right > width) && (height > top) && (bottom > height)
}

fn fail(this: &MontageContainer, world: &mut impl ItemWorld, globals: &ItemGlobals) {
    if globals.listener_word_204 != 0 {
        world.listener_primary(globals.listener);
        world.own_fail(this.own);
    } else {
        if globals.listener_word_200 != 0 {
            world.listener_secondary(globals.listener);
        }
        world.own_fail(this.own);
    }
}

impl MontageContainer {
    /// Tests the merged source/container rectangles against the viewport and
    /// commits the selected item through the UI collaborators.
    pub fn commit_item(&mut self, world: &mut impl ItemWorld, globals: &ItemGlobals, key: u32) {
        if self.phase != 0 {
            return;
        }

        let source = world.lookup_source(globals.registry, key);
        let own = GeometryObject::Container(self.own);
        let item = GeometryObject::Source(source);

        let source_center_x = world.geometry(item, GeometryValue::CenterX);
        let source_half_width = world.geometry(item, GeometryValue::HalfWidth);
        let source_left = sub(source_center_x, mul(source_half_width, HALF));
        let source_half_width_again = world.geometry(item, GeometryValue::HalfWidth);
        let source_center_x_again = world.geometry(item, GeometryValue::CenterX);
        let source_right = add(source_center_x_again, mul(source_half_width_again, HALF));
        let source_center_y = world.geometry(item, GeometryValue::CenterY);
        let source_half_height = world.geometry(item, GeometryValue::HalfHeight);
        let source_top = sub(source_center_y, mul(source_half_height, HALF));
        let source_half_height_again = world.geometry(item, GeometryValue::HalfHeight);
        let source_center_y_again = world.geometry(item, GeometryValue::CenterY);
        let source_bottom = add(source_center_y_again, mul(source_half_height_again, HALF));

        let own_center_x = world.geometry(own, GeometryValue::CenterX);
        let own_half_width = world.geometry(own, GeometryValue::HalfWidth);
        let own_left = sub(own_center_x, mul(own_half_width, HALF));
        let own_half_width_again = world.geometry(own, GeometryValue::HalfWidth);
        let own_center_x_again = world.geometry(own, GeometryValue::CenterX);
        let own_right = add(own_center_x_again, mul(own_half_width_again, HALF));
        let own_center_y = world.geometry(own, GeometryValue::CenterY);
        let own_half_height = world.geometry(own, GeometryValue::HalfHeight);
        let own_top = sub(own_center_y, mul(own_half_height, HALF));
        let own_half_height_again = world.geometry(own, GeometryValue::HalfHeight);
        let own_center_y_again = world.geometry(own, GeometryValue::CenterY);
        let own_bottom = add(own_center_y_again, mul(own_half_height_again, HALF));

        let left = merge_max(own_left, source_left);
        let right = merge_min(source_right, own_right);
        let top = merge_min(source_top, own_top);
        let bottom = merge_max(own_bottom, source_bottom);
        let height = viewport_edge(globals.viewport.height, globals.viewport.height_scale_bits);
        let width = viewport_edge(globals.viewport.width, globals.viewport.width_scale_bits);

        if !viewport_contains(width, left, right, height, top, bottom) {
            fail(self, world, globals);
            return;
        }
        if (world.gatekeeper(self.list) & 0xff) != 0 {
            fail(self, world, globals);
            return;
        }

        let source_tag = world.source_tag(source);
        let wanted_tag = world.tag_id(ItemTag::SourceIdentity);
        if source_tag == wanted_tag && (world.source_subflag(source) & 0xff) == 0 {
            world.notify(globals.notify_context, ItemTag::Notification);
        }

        world.own_dismiss(self.own);
        let second_source = world.lookup_source(globals.registry, key);
        let index = world.source_index(second_source);
        let sink = world.list_sink(self.list, index);
        if (sink as i32) >= 0 {
            world.post_index(self.list, sink, 1);
        }
        let result = world.list_sink(self.list, key);
        self.cursor = result;
        self.pending = key;
        world.tail_notify(self.cfg, result);
        world.listener_result(globals.listener, result);

        let inner = world.list_inner(self.list);
        let value = world.inner_done(inner);
        let scaled = world.measure(self.src, value);
        let configuration = world.build_configuration(
            globals.registry,
            ItemTag::FloatConfiguration,
            scaled.to_bits(),
        );
        world.release_configuration(configuration);
        let value_again = world.inner_done(inner);
        world.tail_forward(globals.tail_context, value_again);
    }
}

#[cfg(test)]
mod host_tests {
    use super::{merge_max, merge_min, viewport_contains, viewport_edge};

    #[test]
    fn viewport_edge_clamps_finite_values_and_preserves_nan() {
        assert_eq!(viewport_edge(-2, 0x3F80_0000).to_bits(), 0.0f32.to_bits());
        assert_eq!(viewport_edge(2, 0x4000_0000).to_bits(), 1.0f32.to_bits());
        assert_eq!(viewport_edge(1, 0x7F80_0000).to_bits(), 1.0f32.to_bits());
        assert_eq!(
            viewport_edge(i32::MIN, 0x3F80_0000).to_bits(),
            0.0f32.to_bits()
        );
        assert!(viewport_edge(0, 0x7FC0_0042).is_nan());
    }

    #[test]
    fn mixed_edge_merge_keeps_the_source_operand_when_it_is_nan() {
        let source_nan = f32::from_bits(0x7FC0_0042);
        assert_eq!(merge_max(1.0, source_nan).to_bits(), source_nan.to_bits());
        assert_eq!(merge_min(source_nan, 1.0).to_bits(), source_nan.to_bits());
        assert_eq!(merge_max(2.0, 1.0).to_bits(), 2.0f32.to_bits());
        assert_eq!(merge_min(1.0, 2.0).to_bits(), 1.0f32.to_bits());
    }

    #[test]
    fn viewport_gate_is_strict_and_nan_fails() {
        assert!(viewport_contains(0.5, 0.25, 0.75, 0.5, 0.25, 0.75));
        assert!(!viewport_contains(0.5, 0.5, 0.75, 0.5, 0.25, 0.75));
        assert!(!viewport_contains(f32::NAN, 0.25, 0.75, 0.5, 0.25, 0.75));
    }
}
