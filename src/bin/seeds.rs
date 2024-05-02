use anyhow::Result;
use clap::{command, Parser};
use cv::{Contour, Draw, MomentsExt, RectExt, ToInputArrayExt};
use finder::{hsva_to_bgra, Config, Hsb, BLUE, CYAN, GREEN, MAGENTA, RED, WHITE, YELLOW};
use opencv::{
    core::{Vector, CV_8UC1},
    imgcodecs::IMREAD_COLOR,
    imgproc::{
        CHAIN_APPROX_SIMPLE, COLOR_BGR2GRAY, COLOR_BGR2HSV, DIST_L2, DIST_MASK_5, FILLED,
        FONT_HERSHEY_SIMPLEX, RETR_EXTERNAL, THRESH_BINARY_INV, THRESH_OTSU,
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

    // Read
    let source = Mat::read(&cli.path, IMREAD_COLOR)?;
    if source.empty() {
        // cli.print_help();
        println!("Source image is empty");
        exit(1);
    }
    // HSV
    let hsv = source.convert_color(COLOR_BGR2HSV)?;
    // Gray
    let gray = source.convert_color(COLOR_BGR2GRAY)?;
    // Threshold
    let binary = gray.threshold(0.0, 255.0, THRESH_BINARY_INV | THRESH_OTSU)?;
    // Contours
    let mut contours = binary.find_contours(RETR_EXTERNAL, CHAIN_APPROX_SIMPLE)?;

    // Filter
    let mut filtered = (Vector::new(), Vector::<Mat>::new());
    for contour in contours {
        if contour.area()? >= config.contours.min_area {
            filtered.0.push(contour);
        } else {
            filtered.1.push(contour);
        }
    }
    let mut filter = source.clone();
    filter.draw_contours(&filtered.0, GREEN, 1)?;
    filter.draw_contours(&filtered.1, RED, 1)?;
    filter.write(cli.path.with_extension("filter.png"))?;
    contours = filtered.0;

    // Process
    let mut seeds = Vec::new();
    let mut contoured = source.clone();
    for (index, contour) in contours.iter().enumerate() {
        let perimeter = contour.perimeter(true)?;
        let area = contour.area()?;

        // Contour
        let centroid = contour.moments(false)?.centroid();
        contoured.draw_contour(&contour, RED, 1)?;
        contoured.draw_circle(centroid, 1, RED, 1)?;

        // Bounding rectangle
        let bounding_rectangle = contour.bounding_rectangle()?;
        contoured.draw_rectangle(bounding_rectangle, YELLOW)?;
        contoured.draw_circle(bounding_rectangle.center(), 1, YELLOW, 1)?;

        // Rotated rectangle
        let rotated_rectangle = contour.rotated_rectangle()?;
        contoured.draw_rotated_rectangle(rotated_rectangle, GREEN)?;
        contoured.draw_circle(rotated_rectangle.center, 1, GREEN, 1)?;

        // Min circumcircle
        let min_circumcircle = contour.min_circumcircle()?;
        contoured.draw_circle(min_circumcircle.center, min_circumcircle.radius, BLUE, 1)?;
        contoured.draw_circle(min_circumcircle.center, 1, BLUE, 1)?;

        // Max incircle
        let max_incircle = contour.max_incircle()?;
        contoured.draw_circle(max_incircle.center, max_incircle.radius, CYAN, 1)?;
        contoured.draw_circle(max_incircle.center, 1, CYAN, 1)?;

        // Incircle
        let incircle = contour.incircle(centroid)?;
        contoured.draw_circle(incircle.center, incircle.radius, MAGENTA, 1)?;
        contoured.draw_circle(incircle.center, 1, MAGENTA, 1)?;

        // Mean colors
        let colors = {
            let radius = incircle.radius / 2.0;
            // RED
            let contour = {
                let mut mask = Mat::zeros_size(source.size()?, CV_8UC1)?.to_mat()?;
                mask.draw_contour(&contour, WHITE, FILLED)?;
                let hsva = hsv.mean(&mut mask)?;
                contoured.draw_circle(
                    bounding_rectangle.tl(),
                    radius,
                    hsva_to_bgra(hsva)?,
                    FILLED,
                )?;
                contoured.draw_circle(bounding_rectangle.tl(), radius, RED, 1)?;
                hsva.into()
            };

            // CYAN
            let max_incircle = {
                let mut mask = Mat::zeros_size(source.size()?, CV_8UC1)?.to_mat()?;
                mask.draw_circle(max_incircle.center, max_incircle.radius, WHITE, FILLED)?;
                let hsva = hsv.mean(&mut mask)?;
                contoured.draw_circle(
                    bounding_rectangle.tr(),
                    radius,
                    hsva_to_bgra(hsva)?,
                    FILLED,
                )?;
                contoured.draw_circle(bounding_rectangle.tr(), radius, CYAN, 1)?;
                hsva.into()
            };

            // MAGENTA
            let incircle = {
                let mut mask = Mat::zeros_size(source.size()?, CV_8UC1)?.to_mat()?;
                mask.draw_circle(incircle.center, incircle.radius, WHITE, FILLED)?;
                let hsva = hsv.mean(&mut mask)?;
                contoured.draw_circle(
                    bounding_rectangle.br(),
                    radius,
                    hsva_to_bgra(hsva)?,
                    FILLED,
                )?;
                contoured.draw_circle(bounding_rectangle.br(), radius, MAGENTA, 1)?;
                hsva.into()
            };

            Colors {
                contour,
                max_incircle,
                incircle,
            }
        };

        contoured.draw_text(
            index.to_string(),
            centroid,
            FONT_HERSHEY_SIMPLEX,
            0.5,
            WHITE,
        )?;
        seeds.push(Seed {
            area,
            circumcircle_radius: min_circumcircle.radius as _,
            incircle_radius: max_incircle.radius as _,
            perimeter,
            colors,
        });
    }
    contoured.write(cli.path.with_extension("contoured.png"))?;
    // Test
    let mut mask = Mat::zeros_size(source.size()?, CV_8UC1)?.to_mat()?;
    mask.draw_contours(&contours, WHITE, FILLED)?;
    let distance_transform = mask.distance_transform(DIST_L2, DIST_MASK_5)?;
    distance_transform.write(cli.path.with_extension("distance_transform.png"))?;

    to_writer_pretty(
        File::create(cli.path.with_extension("ron"))?,
        &seeds,
        PrettyConfig::new().depth_limit(1),
    )?;
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
