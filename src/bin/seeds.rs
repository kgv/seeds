use anyhow::Result;
use clap::{command, Parser};
use finder::{Config, Hsb, BLUE, CYAN, GREEN, MAGENTA, RED, WHITE, YELLOW};
use opencv::{
    core::{mean, min_max_loc, no_array, Point2d, Point2f, Point2i, Rect, Vector, CV_8UC1},
    imgcodecs::{imread, imwrite_def, IMREAD_COLOR},
    imgproc::{
        arc_length, bounding_rect, circle, circle_def, contour_area_def, cvt_color_def,
        distance_transform_def, draw_contours, draw_contours_def, find_contours_def, line_def,
        min_area_rect, min_enclosing_circle, moments, point_polygon_test, put_text_def,
        rectangle_def, threshold, CHAIN_APPROX_SIMPLE, COLOR_BGR2GRAY, COLOR_BGR2HSV, DIST_L2,
        DIST_MASK_5, FILLED, FONT_HERSHEY_SIMPLEX, RETR_EXTERNAL, THRESH_BINARY_INV, THRESH_OTSU,
    },
    prelude::*,
};
use ron::ser::{to_writer_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf, process::exit};

#[derive(Parser)]
#[command(about, arg_required_else_help = true, long_about = None, version)]
struct Cli {
    /// Path to source image
    path: PathBuf,

    /// Sets a custom config file
    #[arg(short, long, value_name = "CONFIG")]
    config: Option<PathBuf>,
    // /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,
}

// let path = "assets/images/water_coins.jpg";
// cargo run -- "assets/images/20240416_164427.jpg"
fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = if let Some(path) = &cli.config {
        match Config::new(path) {
            Ok(config) => config,
            Err(error) => {
                println!("Failed to load config: {error}");
                exit(1);
            }
        }
    } else {
        Config::default()
    };
    to_writer_pretty(File::create("config.ron")?, &config, PrettyConfig::new())?;
    let path = cli.path.to_str().expect("expected valid source image path");

    // Read the image
    let source = imread(path, IMREAD_COLOR)?;
    if source.empty() {
        // cli.print_help();
        println!("Source image is empty");
        exit(1);
    }
    // println!("{source:?}");
    // let p: &VecN<u8, 3> = source.at_2d(444, 1111)?;
    // println!("BGR: {p:?}");

    let mut hsv = Mat::default();
    cvt_color_def(&source, &mut hsv, COLOR_BGR2HSV)?;
    // println!("{hsv:?}");
    // let p: &VecN<u8, 3> = hsv.at_2d(444, 1111)?;
    // println!("HSV: {p:?}");
    // imwrite_def("1.hsv.png", &hsv)?;
    // imshow("1.hsv.png", );

    // Gray
    let mut grayed = Mat::default();
    cvt_color_def(&source, &mut grayed, COLOR_BGR2GRAY)?;
    // Threshold
    let mut thresh = Mat::default();
    threshold(
        &grayed,
        &mut thresh,
        0.0,
        255.0,
        THRESH_BINARY_INV | THRESH_OTSU,
    )?;
    // Contours
    let mut contours = Vector::<Mat>::new();
    find_contours_def(&thresh, &mut contours, RETR_EXTERNAL, CHAIN_APPROX_SIMPLE)?;

    // Filter
    let mut filtered_image = source.clone();
    let mut filtered_contours = Vector::new();
    let mut areas = Vec::new();
    for (index, contour) in contours.iter().enumerate() {
        let area = contour_area_def(&contour)?;
        if area < config.contours.min_area {
            draw_contours_def(&mut filtered_image, &contours, index as _, RED)?;
        } else {
            draw_contours_def(&mut filtered_image, &contours, index as _, GREEN)?;
            filtered_contours.push(contour);
            areas.push(area);
        }
    }
    let path = cli.path.with_extension("filtered.png");
    imwrite_def(path.to_str().unwrap(), &filtered_image)?;
    contours = filtered_contours;

    // Process
    let mut seeds = Vec::new();
    let mut contoured = source.clone();
    for (index, contour) in contours.iter().enumerate() {
        let perimeter = arc_length(&contour, true)?;

        // Contour and centroid
        let centroid = {
            draw_contours_def(&mut contoured, &contours, index as _, RED)?;
            let moments = moments(&contour, false)?;
            // Calculate coordinates of centroid
            let centroid = Point2d::new(moments.m10 / moments.m00, moments.m01 / moments.m00);
            circle_def(&mut contoured, centroid.to().unwrap(), 1, RED)?;
            centroid
        };

        // Bounding rectangle
        let bounding_rect = {
            let bounding_rect = bounding_rect(&contour)?;
            rectangle_def(&mut contoured, bounding_rect, YELLOW)?;
            let center = Point2i::new(
                bounding_rect.x + bounding_rect.width / 2,
                bounding_rect.y + bounding_rect.height / 2,
            );
            circle_def(&mut contoured, center, 1, YELLOW)?;
            bounding_rect
        };

        // Rotated rectangle
        {
            let rotated_rect = min_area_rect(&contour)?;
            let mut points = Vector::with_capacity(4);
            rotated_rect.points_vec(&mut points)?;
            for index in 0..4 {
                line_def(
                    &mut contoured,
                    points.get(index)?.to().unwrap(),
                    points.get((index + 1) % 4)?.to().unwrap(),
                    GREEN,
                )?;
            }
            circle_def(&mut contoured, rotated_rect.center.to().unwrap(), 1, GREEN)?;
        }

        // Min circumcircle
        let min_circumcircle = {
            let mut center = Point2f::default();
            let mut radius = 0.0;
            min_enclosing_circle(&contour, &mut center, &mut radius)?;
            let center = center.to().unwrap();
            circle_def(&mut contoured, center, radius as _, BLUE)?;
            circle_def(&mut contoured, center, 1, BLUE)?;
            Circle {
                center,
                radius: radius as f64,
            }
        };

        // Max incircle
        let max_incircle = {
            let mut mask = Mat::zeros_size(source.size()?, CV_8UC1)?.to_mat()?;
            draw_contours(
                &mut mask,
                &contours,
                index as _,
                WHITE,
                FILLED,
                Default::default(),
                &no_array(),
                Default::default(),
                Default::default(),
            )?;
            // Distance Trasnsform
            let mut distance_transform = Mat::default();
            distance_transform_def(&mask, &mut distance_transform, DIST_L2, DIST_MASK_5)?;
            let mut center = Point2i::default();
            let mut radius = 0.0;
            // The max value is the radius, its position is the center
            min_max_loc(
                &distance_transform,
                None,
                Some(&mut radius),
                None,
                Some(&mut center),
                &mask,
            )?;
            circle_def(&mut contoured, center, radius as _, CYAN)?;
            circle_def(&mut contoured, center, 1, CYAN)?;
            Circle { center, radius }
        };

        // Incircle
        let incircle = {
            let radius = point_polygon_test(&contour, centroid.to().unwrap(), true)?;
            let center = centroid.to().unwrap();
            circle_def(&mut contoured, center, radius as _, MAGENTA)?;
            circle_def(&mut contoured, center, 1, MAGENTA)?;
            Circle { center, radius }
        };

        // 3 Approx poly
        // let epsilon = 0.002 * perimeter;
        // let mut approx_curve = Mat::default();
        // approx_poly_dp(&contour, &mut approx_curve, epsilon, true)?;
        // draw_contours_def(&mut contoured, &approx_curve, FILLED, BLUE)?;

        // Mean colors
        let colors = {
            let radius = (incircle.radius / 2.0) as _;
            // RED
            let mut mask = Mat::zeros_size(source.size()?, CV_8UC1)?.to_mat()?;
            draw_contours(
                &mut mask,
                &contours,
                index as _,
                WHITE,
                FILLED,
                Default::default(),
                &no_array(),
                Default::default(),
                Default::default(),
            )?;
            let means = mean(&hsv, &mut mask)?;
            let contour = Hsb {
                hue: means[0],
                saturation: means[1],
                brightness: means[2],
            };
            let bgr = contour.bgr()?;
            circle(
                &mut contoured,
                bounding_rect.tl(),
                radius,
                bgr.into(),
                FILLED,
                Default::default(),
                Default::default(),
            )?;
            circle_def(&mut contoured, bounding_rect.tl(), radius, RED)?;

            // CYAN
            let mut mask = Mat::zeros_size(source.size()?, CV_8UC1)?.to_mat()?;
            circle(
                &mut mask,
                max_incircle.center,
                max_incircle.radius as _,
                WHITE,
                FILLED,
                Default::default(),
                Default::default(),
            )?;
            let means = mean(&hsv, &mut mask)?;
            let max_incircle = Hsb {
                hue: means[0],
                saturation: means[1],
                brightness: means[2],
            };
            let bgr = max_incircle.bgr()?;
            circle(
                &mut contoured,
                bounding_rect.tr(),
                radius,
                bgr.into(),
                FILLED,
                1,
                Default::default(),
            )?;
            circle_def(&mut contoured, bounding_rect.tr(), radius, CYAN)?;

            // MAGENTA
            let mut mask = Mat::zeros_size(source.size()?, CV_8UC1)?.to_mat()?;
            circle(
                &mut mask,
                incircle.center,
                incircle.radius as _,
                WHITE,
                FILLED,
                Default::default(),
                Default::default(),
            )?;
            let means = mean(&hsv, &mut mask)?;
            let incircle = Hsb {
                hue: means[0],
                saturation: means[1],
                brightness: means[2],
            };
            let bgr = incircle.bgr()?;
            circle(
                &mut contoured,
                bounding_rect.br(),
                radius,
                bgr.into(),
                FILLED,
                1,
                Default::default(),
            )?;
            circle_def(&mut contoured, bounding_rect.br(), radius, MAGENTA)?;

            Colors {
                contour,
                max_incircle,
                incircle,
            }
        };

        put_text_def(
            &mut contoured,
            &index.to_string(),
            centroid.to().unwrap(),
            FONT_HERSHEY_SIMPLEX,
            0.5,
            WHITE,
        )?;
        seeds.push(Seed {
            area: areas[index],
            circumcircle_radius: min_circumcircle.radius,
            incircle_radius: max_incircle.radius,
            perimeter,
            colors,
        });
    }
    let path = cli.path.with_extension("contoured.png");
    imwrite_def(path.to_str().unwrap(), &contoured)?;
    to_writer_pretty(
        File::create(cli.path.with_extension("ron"))?,
        &seeds,
        PrettyConfig::new().depth_limit(1),
    )?;

    // let mut morphologized = Mat::default();
    // let mut kernel = Vector::<Mat>::new();
    // morphology_ex_def(&thresh, &mut morphologized, MORPH_DILATE, &no_array())?;
    // imwrite_def("4.morphologized.png", &morphologized)?;

    // watershed(&source, &mut contoured)?;
    Ok(())
}

/// Seed
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
struct Seed {
    area: f64,
    circumcircle_radius: f64,
    incircle_radius: f64,
    perimeter: f64,
    colors: Colors,
}

/// Colors
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
struct Colors {
    contour: Hsb,
    max_incircle: Hsb,
    incircle: Hsb,
}

/// Circle
#[derive(Clone, Copy, Debug, Default)]
struct Circle {
    center: Point2i,
    radius: f64,
}

trait RectExt {
    fn tr(&self) -> Point2i;
}

impl RectExt for Rect {
    fn tr(&self) -> Point2i {
        Point2i::new(self.x + self.width, self.y)
    }
}

// fn probabilistic_hough(edges: &Mat) -> Result<()> {
//     let mut p_lines = VectorOfVec4i::new();
//     let mut probabalistic_hough = Mat::default();
//     cvt_color_def(edges, &mut probabalistic_hough, COLOR_GRAY2BGR)?;
//     // 2. Use Probabilistic Hough Transform
//     hough_lines_p(
//         edges,
//         &mut p_lines,
//         1.,
//         PI / 180.,
//         MIN_THRESHOLD + p_trackbar,
//         30.,
//         10.,
//     )?;
//     // Show the result
//     for l in p_lines {
//         line(
//             &mut probabalistic_hough,
//             Point::new(l[0], l[1]),
//             Point::new(l[2], l[3]),
//             (255, 0, 0).into(),
//             3,
//             LINE_AA,
//             0,
//         )?;
//     }
//     imshow(PROBABILISTIC_NAME, &probabalistic_hough)?;
//     Ok(())
// }
