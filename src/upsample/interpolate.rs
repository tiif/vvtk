use std::collections::HashSet;

use kiddo::{distance::squared_euclidean, KdTree};
use log::warn;

use crate::formats::{pointxyzrgba::PointXyzRgba, PointCloud};

pub fn upsample(point_cloud: PointCloud<PointXyzRgba>, factor: usize) -> PointCloud<PointXyzRgba> {
    if factor <= 1 {
        point_cloud
    } else {
        let mut points = point_cloud.points;
        let neighbour_radius = factor as f32 * 2.0 * 9.0;
        let mut kd_tree = KdTree::new();
        for (i, pt) in points.iter().enumerate() {
            kd_tree
                .add(&[pt.x, pt.y, pt.z], i)
                .expect("Failed to add to kd tree");
        }
        let mut new_points = vec![];
        let mut processed = HashSet::new();

        for i in 0..point_cloud.number_of_points {
            processed.insert(i);
            let point = points[i];
            let x = point.x;
            let y = point.y;
            let z = point.z;
            match kd_tree.within(&[x, y, z], neighbour_radius, &squared_euclidean) {
                Ok(nearest) => {
                    for (dist, &idx) in nearest {
                        if processed.contains(&idx) {
                            continue;
                        }

                        let point_n = points[idx];
                        let x_n = point_n.x;
                        let y_n = point_n.y;
                        let z_n = point_n.z;

                        let x_diff = x_n - x;
                        let y_diff = y_n - y;
                        let z_diff = z_n - z;

                        for k in 1..=(2 * factor) {
                            let k = k as f32;
                            let factor = factor as f32;
                            let scale = k / (2.0 * factor);

                            let geom_x = x + (x_diff * scale);
                            let geom_y = y + (y_diff * scale);
                            let geom_z = z + (z_diff * scale);

                            let pi_dist = dist * scale;
                            let ni_dist = dist - pi_dist;

                            let col_r = ((point.r as f32) * pi_dist + (point_n.r as f32) * ni_dist)
                                * (1.0 / dist);
                            let col_g = ((point.g as f32) * pi_dist + (point_n.g as f32) * ni_dist)
                                * (1.0 / dist);
                            let col_b = ((point.b as f32) * pi_dist + (point_n.b as f32) * ni_dist)
                                * (1.0 / dist);
                            let col_a = ((point.a as f32) * pi_dist + (point_n.a as f32) * ni_dist)
                                * (1.0 / dist);
                            new_points.push(PointXyzRgba {
                                x: geom_x,
                                y: geom_y,
                                z: geom_z,
                                r: col_r as u8,
                                g: col_g as u8,
                                b: col_b as u8,
                                a: col_a as u8,
                            })
                        }
                    }
                }
                Err(e) => {
                    warn!("{:?}", e);
                    continue;
                }
            }
        }
        points.extend(new_points);
        PointCloud {
            number_of_points: points.len(),
            points,
        }
    }
}