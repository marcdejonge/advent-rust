#![feature(test)]
#![feature(iter_map_windows)]

use std::cmp::Reverse;

use advent_lib::*;
use rayon::prelude::*;

type Point = advent_lib::geometry::Point<2, i32>;
type Line = advent_lib::lines::LineSegment<2, i32>;
type BoundingBox = advent_lib::geometry::BoundingBox<2, i32>;

fn find_area<F>(points: &[Point], predicate: F) -> (i64, BoundingBox)
where
    F: Fn(&BoundingBox) -> bool + Send + Sync,
{
    points
        .iter()
        .enumerate()
        .par_bridge()
        .flat_map_iter(|(ix, left)| {
            points.iter().skip(ix).map(move |right| BoundingBox::from(*left, *right))
        })
        .filter(predicate)
        .map(|bb| {
            let v = bb.max_point() - bb.min_point();
            ((v.x() + 1) as i64 * (v.y() + 1) as i64, bb)
        })
        .max()
        .unwrap()
}

fn calculate_part1(points: &[Point]) -> i64 {
    #[allow(unused_variables)]
    let (area, bb) = find_area(points, |_| true);

    #[cfg(feature = "generate_image")]
    render_lines_and_bounding_box(&points, bb, "day9_1.svg").expect("Could not write file");

    area
}

fn calculate_part2(points: &[Point]) -> i64 {
    let mut lines: Vec<_> = points
        .iter()
        .chain(points.iter().take(1)) // Chain the first one to create the last to first line segment
        .map_windows(|[p1, p2]| Line::from((**p1, **p2)))
        .collect();

    // Add lines to corners that make this polygon non-convex, to prevent us selecting boxes that
    // are completely outside of this shape.
    let len = lines.len() - 1;
    for ix in 0..len {
        if let Some(dir1) = lines[ix].get_direction()
            && let Some(dir2) = lines[ix + 1].get_direction()
            // ASSUMPTION: The polygon is going clockwise
            && dir1.turn_left() == dir2
        {
            // Inner corner, make sure we add a line across it to make sure we don't put a box outside
            lines.push(Line::from((lines[ix].start, lines[ix + 1].end)))
        }
    }

    // Sort the lines to check the long lines first, which have the highest change of crossing
    lines.sort_unstable_by_key(|line| Reverse((line.end - line.start).euler()));

    #[allow(unused_variables)]
    let (area, bb) = find_area(points, |bb| {
        // There should be no line that crosses the boundary box
        !lines.iter().any(|line| {
            line.start.x().max(line.end.x()) > bb.min_point().x()
                && line.start.x().min(line.end.x()) < bb.max_point().x()
                && line.start.y().max(line.end.y()) > bb.min_point().y()
                && line.start.y().min(line.end.y()) < bb.max_point().y()
        })
    });

    #[cfg(feature = "generate_image")]
    render_lines_and_bounding_box(&points, bb, "day9_2.svg").expect("Could not write file");

    area
}

day_main!(Vec<Point>);

day_test!( 9, example => 50, 24 );
day_test!( 9, simple => 81, 27 );
day_test!( 9 => 4774877510, 1560475800 );

#[cfg(feature = "generate_image")]
fn render_lines_and_bounding_box(
    points: &[Point],
    bb: BoundingBox,
    name: &str,
) -> std::io::Result<()> {
    use itertools::Itertools;
    use std::{fs::File, io::BufWriter, io::Write};

    let (min_x, max_x) = points.iter().map(|p| p.x()).minmax().into_option().unwrap();
    let (min_y, max_y) = points.iter().map(|p| p.y()).minmax().into_option().unwrap();
    let width = max_x - min_x;
    let height = max_y - min_y;
    let line_width = width.max(height) as f32 / 500f32;

    let mut w = BufWriter::new(File::create(name)?);
    write!(
        w,
        "<svg version=\"1.1\" viewBox=\"{} {} {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
        min_x as f32 - 10. * line_width,
        min_y as f32 - 10. * line_width,
        width as f32 + 20. * line_width,
        height as f32 + 20. * line_width
    )?;
    write!(
        w,
        "<style>
        path {{ stroke: black; stroke-width: {}; fill: #e0e0e0; }}
        rect {{ fill: rgba(0, 0, 180, .5); }}
        </style>",
        line_width,
    )?;
    write!(w, "<path d=\"M {} {}", points[0].x(), points[0].y())?;
    for p in &points[1..] {
        write!(w, " L {} {}", p.x(), p.y(),)?;
    }
    write!(w, " Z\"/>")?;

    let size = bb.total_size();
    write!(
        w,
        "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" />",
        bb.min_point().x(),
        bb.min_point().y(),
        size.x(),
        size.y()
    )?;
    write!(w, "</svg>")?;

    Ok(())
}
