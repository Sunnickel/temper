use bevy_ecs::prelude::Component;
use bevy_math::bounding::{Aabb3d, BoundingVolume, IntersectsVolume};
use bevy_math::Vec3A;
use deref_derive::{Deref, DerefMut};

/// Entity bounding box (collision box).
///
/// Represents the volume occupied by an entity in the world.
/// Used for collision detection and physics.
#[derive(Component, Deref, DerefMut, Copy, Clone, Debug)]
pub struct CollisionBounds(Aabb3d);

impl Default for CollisionBounds {
    fn default() -> Self {
        CollisionBounds(Aabb3d {
            min: Vec3A::new(0.0, 0.0, 0.0),
            max: Vec3A::new(0.0, 0.0, 0.0),
        })
    }
}

impl CollisionBounds {
    pub fn new(min: Vec3A, max: Vec3A) -> Self {
        CollisionBounds(Aabb3d { min, max })
    }

    #[inline]
    pub fn collides(
        &self,
        own_pos: Vec3A,
        other_bounds: &CollisionBounds,
        other_pos: Vec3A,
    ) -> bool {
        self.translated_by(own_pos)
            .intersects(&other_bounds.translated_by(other_pos))
    }
    /// Creates a new bounding box from vanilla dimensions.
    ///
    /// # Arguments
    ///
    /// * `dimension` - Dimensions [width, height] from temper-data
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use temper_data::generated::entities::EntityType as VanillaEntityType;
    ///
    /// let bbox = BoundingBox::from_vanilla_dimension(VanillaEntityType::PIG.dimension);
    /// assert_eq!(bbox.half_width, 0.45); // 0.9 / 2
    /// assert_eq!(bbox.height, 0.9);
    /// ```
    pub const fn from_vanilla_dimension(dimension: [f32; 2]) -> Self {
        Self(Aabb3d {
            max: Vec3A::new(dimension[0] / 2.0, dimension[1], dimension[0] / 2.0),
            min: Vec3A::new(-(dimension[0] / 2.0), 0.0, -(dimension[0] / 2.0)),
        })
    }

    /// Returns the total width of the bounding box.
    pub fn width(&self) -> f64 {
        f64::from(self.max.x - self.min.x)
    }

    /// Returns the height of the bounding box.
    pub fn height(&self) -> f64 {
        f64::from(self.max.y - self.min.y)
    }

    /// Returns the depth of the bounding box.
    pub fn depth(&self) -> f64 {
        f64::from(self.max.z - self.min.z)
    }

    /// Returns the volume of the bounding box in cubic blocks.
    pub fn volume(&self) -> f64 {
        self.width() * self.height() * self.depth()
    }
}

impl From<Aabb3d> for CollisionBounds {
    fn from(aabb: Aabb3d) -> Self {
        CollisionBounds(aabb)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collides_when_boxes_overlap() {
        let bounds1 = CollisionBounds::new(Vec3A::new(0.0, 0.0, 0.0), Vec3A::new(1.0, 1.0, 1.0));
        let bounds2 = CollisionBounds::new(Vec3A::new(0.0, 0.0, 0.0), Vec3A::new(1.0, 1.0, 1.0));
        let pos1 = Vec3A::new(0.0, 0.0, 0.0);
        let pos2 = Vec3A::new(0.5, 0.5, 0.5);
        assert!(bounds1.collides(pos1, &bounds2, pos2));
    }

    #[test]
    fn does_not_collide_when_boxes_do_not_overlap() {
        let bounds1 = CollisionBounds::new(Vec3A::new(0.0, 0.0, 0.0), Vec3A::new(1.0, 1.0, 1.0));
        let bounds2 = CollisionBounds::new(Vec3A::new(0.0, 0.0, 0.0), Vec3A::new(1.0, 1.0, 1.0));
        let pos1 = Vec3A::new(0.0, 0.0, 0.0);
        let pos2 = Vec3A::new(3.0, 3.0, 3.0);
        assert!(!bounds1.collides(pos1, &bounds2, pos2));
    }

    #[test]
    fn collides_when_boxes_touch_edges() {
        let bounds1 = CollisionBounds::new(Vec3A::new(0.0, 0.0, 0.0), Vec3A::new(1.0, 1.0, 1.0));
        let bounds2 = CollisionBounds::new(Vec3A::new(0.0, 0.0, 0.0), Vec3A::new(1.0, 1.0, 1.0));
        let pos1 = Vec3A::new(0.0, 0.0, 0.0);
        let pos2 = Vec3A::new(1.0, 0.0, 0.0);
        assert!(bounds1.collides(pos1, &bounds2, pos2));
    }

    #[test]
    fn collides_when_one_box_inside_another() {
        let bounds1 = CollisionBounds::new(Vec3A::new(0.0, 0.0, 0.0), Vec3A::new(2.0, 2.0, 2.0));
        let bounds2 = CollisionBounds::new(Vec3A::new(0.0, 0.0, 0.0), Vec3A::new(1.0, 1.0, 1.0));
        let pos1 = Vec3A::new(0.0, 0.0, 0.0);
        let pos2 = Vec3A::new(0.5, 0.5, 0.5);
        assert!(bounds1.collides(pos1, &bounds2, pos2));
    }

    #[test]
    fn does_not_collide_when_positions_are_far_apart() {
        let bounds1 = CollisionBounds::new(Vec3A::new(0.0, 0.0, 0.0), Vec3A::new(1.0, 1.0, 1.0));
        let bounds2 = CollisionBounds::new(Vec3A::new(0.0, 0.0, 0.0), Vec3A::new(1.0, 1.0, 1.0));
        let pos1 = Vec3A::new(0.0, 0.0, 0.0);
        let pos2 = Vec3A::new(10.0, 10.0, 10.0);
        assert!(!bounds1.collides(pos1, &bounds2, pos2));
    }
}
