//! Portable lift of the indexed montage pick and commit.

use super::container::MontageContainer;
use super::item_commit::{
    add, div, merge_max, merge_min, mul, sub, viewport_contains, viewport_edge,
};
use super::item_world::{GeometryObject, GeometryValue, ItemGlobals, ItemTag, ItemWorld};

const HALF: f32 = f32::from_bits(0x3F00_0000);

struct Rectangle {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    last_getter: f32,
}

fn rectangle(world: &mut impl ItemWorld, object: GeometryObject) -> Rectangle {
    let center_x = world.geometry(object, GeometryValue::CenterX);
    let half_width = world.geometry(object, GeometryValue::HalfWidth);
    let left = sub(center_x, mul(half_width, HALF));
    let half_width_again = world.geometry(object, GeometryValue::HalfWidth);
    let center_x_again = world.geometry(object, GeometryValue::CenterX);
    let right = add(center_x_again, mul(half_width_again, HALF));
    let center_y = world.geometry(object, GeometryValue::CenterY);
    let half_height = world.geometry(object, GeometryValue::HalfHeight);
    let top = sub(center_y, mul(half_height, HALF));
    let half_height_again = world.geometry(object, GeometryValue::HalfHeight);
    let center_y_again = world.geometry(object, GeometryValue::CenterY);
    let bottom = add(center_y_again, mul(half_height_again, HALF));
    Rectangle {
        left,
        right,
        top,
        bottom,
        last_getter: center_y_again,
    }
}

fn rectangles_inside(source: &Rectangle, container: &Rectangle, width: f32, height: f32) -> bool {
    let left = merge_max(container.left, source.left);
    let right = merge_min(source.right, container.right);
    let top = merge_min(source.top, container.top);
    let bottom = merge_max(container.bottom, source.bottom);
    viewport_contains(width, left, right, height, top, bottom)
}

fn signed_word_as_float(value: u32) -> f32 {
    (value as i32) as f32
}

impl MontageContainer {
    /// Picks and commits the indexed montage item using the mixed-polarity
    /// rectangle gate, signed quotient, and unsigned index update.
    pub fn indexed_pick(
        &mut self,
        world: &mut impl ItemWorld,
        globals: &ItemGlobals,
        key: u32,
    ) -> u32 {
        if world.list_current_key(self.list) == key {
            let sub = world.list_subflag(self.list);
            if (sub & 0xFF) != 0 {
                return sub;
            }
            let probe = world.list_probe(self.list);
            if probe == 0 {
                return 0;
            }
            if (((globals.mask_b ^ globals.mask_a) & globals.mask_b) & 1) == 0 {
                return probe;
            }
            return world.own_dismiss(self.own);
        }

        let source = world.lookup_source(globals.registry, key);
        let source_rectangle = rectangle(world, GeometryObject::Source(source));
        let container_rectangle = rectangle(world, GeometryObject::Container(self.own));
        let height = viewport_edge(globals.viewport.height, globals.viewport.height_scale_bits);
        let width = viewport_edge(globals.viewport.width, globals.viewport.width_scale_bits);
        let gate_one = rectangles_inside(&source_rectangle, &container_rectangle, width, height);
        if gate_one {
            let sink = world.list_sink(self.list, key);
            self.want = sink;
            let center_x = world.geometry(GeometryObject::Source(source), GeometryValue::CenterX);
            let half_width =
                world.geometry(GeometryObject::Source(source), GeometryValue::HalfWidth);
            let distance = sub(width, sub(center_x, mul(half_width, HALF)));
            let random_one = world.random_word();
            let product = mul(signed_word_as_float(random_one), distance);
            let half_width_again =
                world.geometry(GeometryObject::Source(source), GeometryValue::HalfWidth);
            let random_two = world.random_word();
            let denominator = mul(half_width_again, signed_word_as_float(random_two));
            let quotient = div(product, denominator);
            self.quotient_bits = quotient.to_bits();
            if quotient > 0.5 {
                let count = world.list_probe(self.list);
                let index = self.want;
                self.want = if index < count {
                    index.wrapping_add(1)
                } else {
                    u32::MAX
                };
            }
        }

        let source_tag = world.source_tag(source);
        let wanted_tag = world.tag_id(ItemTag::SourceIdentity2);
        if source_tag != wanted_tag {
            return wanted_tag;
        }

        let source_rectangle = rectangle(world, GeometryObject::Source(source));
        let container_rectangle = rectangle(world, GeometryObject::Container(self.own));
        if !rectangles_inside(&source_rectangle, &container_rectangle, width, height) {
            return container_rectangle.last_getter.to_bits();
        }

        let first_answer = world.listener_refresh(globals.listener, 2);
        let Some(epilogue) = self.epilogue else {
            return first_answer;
        };
        world.epilogue_set(epilogue, 0xE);
        world.epilogue_finish(epilogue, 1, 1)
    }
}

#[cfg(test)]
mod host_tests {
    use super::signed_word_as_float;

    #[test]
    fn random_word_conversion_is_signed() {
        assert_eq!(signed_word_as_float(0x7FFF_FFFF), i32::MAX as f32);
        assert_eq!(signed_word_as_float(0x8000_0000), i32::MIN as f32);
        assert_eq!(signed_word_as_float(u32::MAX), -1.0);
    }

    #[test]
    fn quotient_threshold_is_strict_and_nan_is_false() {
        assert!(!(0.5f32 > 0.5));
        assert!(f32::from_bits(0x3F00_0001) > 0.5);
        let nan = f32::from_bits(0x7FC0_0042);
        assert!(!(nan > 0.5));
    }
}
