pub mod areas_volumes;

pub use areas_volumes::*;


pub fn area_fit(
    x: usize,
    y: usize,
    objects: GeometricalShapes,
    times: usize,
    a: usize,
    b: usize,
) -> bool {
    let area = x * y;
    let required_area = match objects {
        GeometricalShapes::Square => areas_volumes::square_area(a) as f64,
        GeometricalShapes::Rectangle => areas_volumes::rectangle_area(a, b) as f64,
        GeometricalShapes::Triangle => areas_volumes::triangle_area(a, b),
        GeometricalShapes::Circle => areas_volumes::circle_area(a),
    };
    (required_area * times as f64) <= area as f64
}

pub fn volume_fit(
    x: usize,
    y: usize,
    z: usize,
    objects: GeometricalVolumes,
    times: usize,
    a: usize,
    b: usize,
    c: usize,
) -> bool {
    let volume = x * y * z;
    let required_volume = match objects {
        GeometricalVolumes::Cube => areas_volumes::cube_volume(a) as f64,
        GeometricalVolumes::Sphere => areas_volumes::sphere_volume(a),
        GeometricalVolumes::Cone => areas_volumes::cone_volume(a, b),
        GeometricalVolumes::Pyramid => areas_volumes::triangular_pyramid_volume(a as f64, b),
        GeometricalVolumes::Parallelepiped => areas_volumes::parallelepiped_volume(a, b, c) as f64,
    };
    (required_volume * times as f64) <= volume as f64
}

