#![feature(test)]
#![feature(iter_map_windows)]

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

fn calculate_part1(points: &[Point]) -> i64 { find_area(points, |_| true).0 }

fn calculate_part2(points: &[Point]) -> i64 {
    let lines: Vec<_> = points
        .iter()
        .chain(points.iter().take(1)) // Chain the first one to create the last to first line segment
        .map_windows(|[p1, p2]| Line::from((**p1, **p2)))
        .collect();

    #[cfg(feature = "generate_image")]
    {
        let (area, bb) = find_area(points, |bb| !lines.iter().any(|line| bb.line_crosses(line)));
        render_lines_and_bounding_box(&lines, bb).expect("Could not write file");
        area
    }

    #[cfg(not(feature = "generate_image"))]
    find_area(points, |bb| !lines.iter().any(|line| bb.line_crosses(line))).0
}

day_main!(Vec<Point>);

day_test!( 9, example => 50, 24 );
day_test!( 9 => 4774877510, 1560475800 );

#[cfg(feature = "generate_image")]
fn render_lines_and_bounding_box(lines: &[Line], bb: BoundingBox) -> std::io::Result<()> {
    use std::{fs::File, io::BufWriter, io::Write};

    let mut w = BufWriter::new(File::create("day9.svg")?);
    write!(
        w,
        "<svg version=\"1.1\" viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
        100_000, 100_000
    )?;
    write!(
        w,
        "<style>
        line {{ stroke: black; stroke-width: 100; }}
        rect {{ fill: blue; }}
        </style>"
    )?;
    for line in lines {
        write!(
            w,
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" />",
            line.start.x(),
            line.start.y(),
            line.end.x(),
            line.end.y()
        )?;
    }
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
