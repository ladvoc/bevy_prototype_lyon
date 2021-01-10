//! Common shapes, like rectangles, ellipses, triangles and more.

use crate::{
    conversions::{ToLyonPoint, ToLyonVector},
    create_sprite, Buffers, ShapeSprite, TessellationMode, Tessellator,
};
use bevy::{
    asset::{Assets, Handle},
    ecs::ResMut,
    math::Vec2,
    render::mesh::Mesh,
    sprite::{entity::SpriteBundle, ColorMaterial},
    transform::components::Transform,
};
use lyon_tessellation::{
    math::{Angle, Point, Rect, Size},
    path::{path::Builder, traits::PathBuilder, Polygon as LyonPolygon, Winding},
};

/// Defines where the origin, or pivot of the `Rectangle` should be positioned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RectangleOrigin {
    Center,
    BottomLeft,
    BottomRight,
    TopRight,
    TopLeft,
}

impl Default for RectangleOrigin {
    fn default() -> Self {
        Self::Center
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub width: f32,
    pub height: f32,
    pub origin: RectangleOrigin,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
            origin: RectangleOrigin::default(),
        }
    }
}

impl ShapeSprite for Rectangle {
    fn generate_sprite(
        &self,
        material: Handle<ColorMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>,
        tessellator: &mut Tessellator,
        mode: TessellationMode,
        transform: Transform,
    ) -> SpriteBundle {
        let mut buffers = Buffers::new();

        use RectangleOrigin::*;
        let origin = match self.origin {
            Center => Point::new(-self.width / 2.0, -self.height / 2.0),
            BottomLeft => Point::new(0.0, 0.0),
            BottomRight => Point::new(-self.width, 0.0),
            TopRight => Point::new(-self.width, -self.height),
            TopLeft => Point::new(0.0, -self.height),
        };

        let mut path_builder = Builder::new();
        path_builder.add_rectangle(
            &Rect::new(origin, Size::new(self.width, self.height)),
            Winding::Positive,
        );
        let path = path_builder.build();

        self.tessellate(&path, &mut buffers, mode, tessellator);

        create_sprite(material, meshes, buffers, transform)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    /// Distance of the border of the circle from the center.
    pub radius: f32,
    /// The position of the center of the circle, relative to the world
    /// [`Translation`] of the [`SpriteBundle`].
    pub center: Vec2,
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            radius: 1.0,
            center: Vec2::zero(),
        }
    }
}

impl ShapeSprite for Circle {
    fn generate_sprite(
        &self,
        material: Handle<ColorMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>,
        tessellator: &mut Tessellator,
        mode: TessellationMode,
        transform: Transform,
    ) -> SpriteBundle {
        let mut buffers = Buffers::new();

        let mut path_builder = Builder::new();
        path_builder.add_circle(self.center.to_lyon_point(), self.radius, Winding::Positive);
        let path = path_builder.build();

        self.tessellate(&path, &mut buffers, mode, tessellator);

        create_sprite(material, meshes, buffers, transform)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ellipse {
    pub radii: Vec2,
    /// The position of the center of the ellipse, relative to the world
    /// [`Translation`] of the [`SpriteBundle`].
    pub center: Vec2,
}

impl Default for Ellipse {
    fn default() -> Self {
        Self {
            radii: Vec2::one(),
            center: Vec2::zero(),
        }
    }
}

impl ShapeSprite for Ellipse {
    fn generate_sprite(
        &self,
        material: Handle<ColorMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>,
        tessellator: &mut Tessellator,
        mode: TessellationMode,
        transform: Transform,
    ) -> SpriteBundle {
        let mut buffers = Buffers::new();

        let mut path_builder = Builder::new();
        path_builder.add_ellipse(
            self.center.to_lyon_point(),
            self.radii.to_lyon_vector(),
            Angle::zero(),
            Winding::Positive,
        );
        let path = path_builder.build();

        self.tessellate(&path, &mut buffers, mode, tessellator);

        create_sprite(material, meshes, buffers, transform)
    }
}

#[derive(Debug, PartialEq)]
pub struct Polygon {
    pub points: Vec<Vec2>,
    pub closed: bool,
}

impl Default for Polygon {
    fn default() -> Self {
        Self {
            points: Vec::new(),
            closed: true,
        }
    }
}

impl ShapeSprite for Polygon {
    fn generate_sprite(
        &self,
        material: Handle<ColorMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>,
        tessellator: &mut Tessellator,
        mode: TessellationMode,
        transform: Transform,
    ) -> SpriteBundle {
        let mut buffers = Buffers::new();

        let points = self
            .points
            .iter()
            .map(|p| p.to_lyon_point())
            .collect::<Vec<Point>>();
        let polygon: LyonPolygon<Point> = LyonPolygon {
            points: points.as_slice(),
            closed: self.closed,
        };

        let mut path_builder = Builder::new();
        path_builder.add_polygon(polygon);
        let path = path_builder.build();

        self.tessellate(&path, &mut buffers, mode, tessellator);

        create_sprite(material, meshes, buffers, transform)
    }
}