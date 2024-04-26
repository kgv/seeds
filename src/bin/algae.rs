use anyhow::anyhow;
use anyhow::Result;
use clap::{command, Parser};
use egui::epaint::util::FloatOrd;
use finder::utils::MatExt;
use finder::Config;
use finder::Hsb;
use finder::GREEN;
use finder::RED;
use itertools::Itertools;
use opencv::core::split;
use opencv::core::CV_8UC4;
use opencv::imgproc::connected_components;
use opencv::imgproc::connected_components_def;
use opencv::imgproc::convex_hull_def;
use opencv::imgproc::DistanceTransformMasks;
use opencv::imgproc::DistanceTypes;
use opencv::imgproc::ThresholdTypes;
use opencv::imgproc::DIST_MASK_3;
use opencv::imgproc::MORPH_CLOSE;
use opencv::{
    core::{
        greater_than_mat_f64, max, mean, min_max_loc, min_max_loc_def, no_array, subtract_def,
        Mat1f, MatExpr, Point2d, Point2f, Point2i, Rect, Scalar, VecN, Vector, CV_8U, CV_8UC1,
    },
    highgui::{imshow, wait_key_def},
    imgcodecs::{imread, imwrite_def, IMREAD_COLOR},
    imgproc::{
        arc_length, bounding_rect, circle, circle_def, contour_area_def, cvt_color_def, dilate,
        distance_transform, distance_transform_def, distance_transform_with_labels,
        distance_transform_with_labels_def, draw_contours, draw_contours_def, find_contours_def,
        line_def, median_blur, min_area_rect, min_enclosing_circle, moments, morphology_ex,
        morphology_ex_def, point_polygon_test, put_text_def, rectangle_def, threshold,
        ADAPTIVE_THRESH_GAUSSIAN_C, CHAIN_APPROX_SIMPLE, COLOR_BGR2GRAY, COLOR_BGR2HSV,
        COLOR_HSV2BGR, DIST_L2, DIST_LABEL_PIXEL, DIST_MASK_5, FILLED, FONT_HERSHEY_SIMPLEX,
        MORPH_DILATE, MORPH_OPEN, RETR_EXTERNAL, THRESH_BINARY, THRESH_BINARY_INV, THRESH_OTSU,
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

    // Source
    let source = imread(path, IMREAD_COLOR)?;
    if source.empty() {
        // cli.print_help();
        println!("Source image is empty");
        exit(1);
    }
    source.write(cli.path.with_extension("source.png"))?;

    let mut temp = Vector::<Mat>::default();
    split(&source, &mut temp)?;
    temp.as_slice()[0].write(cli.path.with_extension("red.png"))?;

    // Gray
    let grayed = source.cvt_color(COLOR_BGR2GRAY)?;

    // Threshold
    let thresholded = grayed.threshold(0.0, 255.0, THRESH_BINARY_INV | THRESH_OTSU)?;
    thresholded.write(cli.path.with_extension("thresholded.png"))?;

    // Remove holes
    let closed = thresholded.morphology(MORPH_CLOSE, &Mat::ones(3, 3, CV_8U)?, 4)?;
    closed.write(cli.path.with_extension("closed.png"))?;

    // Remove noise
    let opened = closed.morphology(MORPH_OPEN, &Mat::ones(3, 3, CV_8U)?, 2)?;
    opened.write(cli.path.with_extension("opened.png"))?;

    // Background area
    let background = opened.dilate(&Mat::ones(3, 3, CV_8U)?, 3)?;
    background.write(cli.path.with_extension("background.png"))?;

    // Foreground area
    let distance_transform = opened.distance_transform(DIST_L2, DIST_MASK_5)?;
    distance_transform.write(cli.path.with_extension("distance_transform.png"))?;

    let tresh = distance_transform
        .iter::<f32>()?
        .max_by_key(|(_, value)| value.ord())
        .unwrap()
        .1 as f64;
    let foreground = distance_transform.threshold(0.5 * tresh, 255.0, THRESH_BINARY)?;
    foreground.write(cli.path.with_extension("foreground.png"))?;

    {
        let mut image = Mat::default();
        foreground.convert_to_def(&mut image, CV_8UC4)?;
        let mut labels = Mat::default();
        connected_components_def(&image, &mut labels)?;

        labels.write(cli.path.with_extension("labels.png"))?;
    }

    // // Finding unknown region
    // let mut unknown = foreground.clone();
    // subtract_def(&background, &foreground, &mut unknown)?;
    // let path = cli.path.with_extension(format!("unknown.png"));
    // imwrite_def(path.to_str().unwrap(), &unknown)?;

    // // Subtract
    // let mut subtracted = Mat::default();
    // subtract_def(&blured, &greater_than, &mut subtracted)?;
    // let path = cli.path.with_extension(format!("subtracted.png"));
    // imwrite_def(path.to_str().unwrap(), &subtracted)?;
    // let max = subtracted
    //     .iter::<u8>()?
    //     .max_by_key(|(_, value)| *value)
    //     .map_or(0, |(_, value)| value);
    // subtracted
    //     .iter_mut::<u8>()?
    //     .for_each(|(_, value)| *value *= (255.0 / max as f64) as u8);
    // let path = cli.path.with_extension(format!("subtracted.1.png"));
    // imwrite_def(path.to_str().unwrap(), &subtracted)?;

    // peakmask = (smooth == cv.dilate(smooth, kernel=None, iterations=20))
    // imshow(peakmask)

    // signal = (signal / signal.max() * 255).astype(np.uint8)
    // imshow(signal)

    // // Remove noise
    // let mut opened = Mat::default();
    // let kernel = Mat::ones(3, 3, CV_8U)?;
    // morphology_ex(
    //     &thresholded,
    //     &mut opened,
    //     MORPH_OPEN,
    //     &kernel,
    //     Default::default(),
    //     2,
    //     Default::default(),
    //     Default::default(),
    // )?;
    // let path = cli.path.with_extension(format!("opened.png"));
    // imwrite_def(path.to_str().unwrap(), &opened)?;

    // // Background area
    // let mut background = Mat::default();
    // dilate(
    //     &opened,
    //     &mut background,
    //     &kernel,
    //     Default::default(),
    //     3,
    //     Default::default(),
    //     Default::default(),
    // )?;
    // let path = cli.path.with_extension(format!("background.png"));
    // imwrite_def(path.to_str().unwrap(), &background)?;

    // // Foreground area
    // let mut foreground = Mat::default();
    // let mut distance_transform = Mat::default();
    // distance_transform_def(&opened, &mut distance_transform, DIST_L2, 5)?;
    // threshold(
    //     &distance_transform,
    //     &mut foreground,
    //     0.7, // * distance_transform.max(),
    //     255.0,
    //     THRESH_BINARY,
    // )?;
    // let path = cli.path.with_extension(format!("foreground.png"));
    // imwrite_def(path.to_str().unwrap(), &foreground)?;

    // // Finding unknown region
    // let mut unknown = foreground.clone();
    // subtract_def(&background, &foreground, &mut unknown)?;
    // let path = cli.path.with_extension(format!("unknown.png"));
    // imwrite_def(path.to_str().unwrap(), &unknown)?;

    // // Contours
    // let mut contours = Vector::<Mat>::new();
    // find_contours_def(
    //     &thresholded,
    //     &mut contours,
    //     RETR_EXTERNAL,
    //     CHAIN_APPROX_SIMPLE,
    // )?;

    // // Filter
    // let mut filtered_image = source.clone();
    // let mut filtered_contours = Vector::new();
    // let mut areas = Vec::new();
    // for (index, contour) in contours.iter().enumerate() {
    //     let area = contour_area_def(&contour)?;
    //     if area < config.contours.min_area {
    //         draw_contours_def(&mut filtered_image, &contours, index as _, RED)?;
    //     } else {
    //         draw_contours_def(&mut filtered_image, &contours, index as _, GREEN)?;
    //         filtered_contours.push(contour);
    //         areas.push(area);
    //     }
    // }
    // let path = cli.path.with_extension("filtered.png");
    // imwrite_def(path.to_str().unwrap(), &filtered_image)?;
    // contours = filtered_contours;

    // let mut morphologized = Mat::default();
    // let mut kernel = Vector::<Mat>::new();
    // morphology_ex_def(&thresh, &mut morphologized, MORPH_DILATE, &no_array())?;
    // imwrite_def("4.morphologized.png", &morphologized)?;

    // watershed(&source, &mut contoured)?;
    Ok(())
}

/// Algae
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
struct Algae {
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
