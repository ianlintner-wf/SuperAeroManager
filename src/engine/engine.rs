extern crate voronoi;
use voronoi::{voronoi, Point, make_polygons};

mod graphics;

pub fn createVoroni() {
  const BOX_SIZE: f64 = 800.;
  let vor_pts = vec![Point::new(0.0, 1.0), Point::new(2.0, 3.0), Point::new(10.0, 12.0)];
  let vor_diagram = voronoi(vor_pts, BOX_SIZE);
  let vor_polys = make_polygons(&vor_diagram);
}