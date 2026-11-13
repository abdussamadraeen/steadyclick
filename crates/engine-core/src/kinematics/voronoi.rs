//! Voronoi Bubble Cursor & Target Acquisition based on Grossman & Balakrishnan (CHI 2005).
//!
//! Maps cursor position to the closest interactive element in motor space,
//! effectively expanding small buttons into large clickable regions according to Fitts' Law.

#![allow(dead_code)]

/// An Axis-Aligned Bounding Box (AABB) for an interactive DOM element.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TargetAABB {
    pub id: u32,
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

impl TargetAABB {
    pub const fn new(id: u32, x0: f64, y0: f64, x1: f64, y1: f64) -> Self {
        Self { id, x0, y0, x1, y1 }
    }

    /// Center point of the target box.
    pub fn center(&self) -> (f64, f64) {
        ((self.x0 + self.x1) * 0.5, (self.y0 + self.y1) * 0.5)
    }

    /// Computes the shortest distance from (px, py) to the boundary of this box.
    /// Returns 0.0 if the point is inside the box.
    pub fn distance_to_point(&self, px: f64, py: f64) -> f64 {
        let dx = if px < self.x0 {
            self.x0 - px
        } else if px > self.x1 {
            px - self.x1
        } else {
            0.0
        };

        let dy = if py < self.y0 {
            self.y0 - py
        } else if py > self.y1 {
            py - self.y1
        } else {
            0.0
        };

        // Euclidean distance
        (dx * dx + dy * dy).sqrt()
    }

    /// Computes the closest point on or inside the box to (px, py).
    pub fn closest_point(&self, px: f64, py: f64) -> (f64, f64) {
        let cx = if px < self.x0 {
            self.x0
        } else if px > self.x1 {
            self.x1
        } else {
            px
        };

        let cy = if py < self.y0 {
            self.y0
        } else if py > self.y1 {
            self.y1
        } else {
            py
        };

        (cx, cy)
    }
}

/// Result of a Voronoi target acquisition query.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AcquisitionResult {
    /// ID of the closest target, or None if no targets exist.
    pub captured_target_id: Option<u32>,
    /// Distance from cursor to the closest target.
    pub distance_to_primary: f64,
    /// Distance to the second closest target (defines maximum safe bubble radius).
    pub distance_to_secondary: f64,
    /// The magnetic coordinate to snap toward.
    pub snap_point: (f64, f64),
}

/// Evaluates targets and determines which element is captured by the Bubble Cursor.
pub fn find_captured_target(
    cursor_x: f64,
    cursor_y: f64,
    targets: &[TargetAABB],
    max_magnetic_radius: f64,
) -> AcquisitionResult {
    if targets.is_empty() {
        return AcquisitionResult {
            captured_target_id: None,
            distance_to_primary: f64::MAX,
            distance_to_secondary: f64::MAX,
            snap_point: (cursor_x, cursor_y),
        };
    }

    let mut min_d1 = f64::MAX;
    let mut min_d2 = f64::MAX;
    let mut best_target: Option<&TargetAABB> = None;

    for target in targets {
        let d = target.distance_to_point(cursor_x, cursor_y);
        if d < min_d1 {
            min_d2 = min_d1;
            min_d1 = d;
            best_target = Some(target);
        } else if d < min_d2 {
            min_d2 = d;
        }
    }

    let target = best_target.expect("target must exist");

    // Magnetic snap interpolation:
    // If within max_magnetic_radius, compute the snap point toward the target center
    let (cx, cy) = target.center();
    let snap_point = if min_d1 <= max_magnetic_radius {
        // Soft magnetism factor
        let factor = 1.0 - (min_d1 / max_magnetic_radius);
        (
            cursor_x + (cx - cursor_x) * factor * 0.7,
            cursor_y + (cy - cursor_y) * factor * 0.7,
        )
    } else {
        (cursor_x, cursor_y)
    };

    AcquisitionResult {
        captured_target_id: Some(target.id),
        distance_to_primary: min_d1,
        distance_to_secondary: min_d2,
        snap_point,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_to_point_inside() {
        let box1 = TargetAABB::new(1, 10.0, 10.0, 50.0, 50.0);
        assert_eq!(box1.distance_to_point(20.0, 30.0), 0.0);
    }

    #[test]
    fn test_distance_to_point_outside() {
        let box1 = TargetAABB::new(1, 10.0, 10.0, 50.0, 50.0);
        // Point is at (53.0, 54.0), dx = 3, dy = 4 -> dist = 5.0
        let dist = box1.distance_to_point(53.0, 54.0);
        assert!((dist - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_find_closest_target() {
        let targets = [
            TargetAABB::new(101, 100.0, 100.0, 120.0, 120.0),
            TargetAABB::new(102, 300.0, 300.0, 350.0, 350.0),
        ];

        // Cursor at (125.0, 120.0) -> close to target 101 (distance 5.0)
        let res = find_captured_target(125.0, 120.0, &targets, 50.0);
        assert_eq!(res.captured_target_id, Some(101));
        assert!((res.distance_to_primary - 5.0).abs() < 1e-6);
        assert!(res.distance_to_secondary > 200.0);
    }
}
