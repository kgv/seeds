use anyhow::{anyhow, Result};
use clap::{command, Parser};
use cv::{Contour, Draw, MatExt, MatTraitConstExt, ToInputArrayExt};
use egui::epaint::util::FloatOrd;
use finder::{Config, Hsb, BLACK, BLUE, CYAN, GREEN, MAGENTA, RED, WHITE, YELLOW};
use image::{
    codecs::png::{
        CompressionType, FilterType,
        FilterType::{Adaptive, NoFilter},
        PngEncoder,
    },
    open, ExtendedColorType, ImageEncoder,
};
use itertools::Itertools;
use kmeans_colors::{get_kmeans_hamerly, Kmeans, MapColor};
use opencv::{
    core::{
        no_array, normalize_def, MatTraitConst, Point, Point2f, Point2i, Rect, Scalar, Size, VecN,
        Vector, CV_32S, CV_8U,
    },
    imgcodecs::{imread, IMREAD_COLOR, IMREAD_GRAYSCALE},
    imgproc::{
        hough_circles, hough_circles_def, match_template, match_template_def, CHAIN_APPROX_SIMPLE,
        COLOR_BGR2GRAY, FILLED, HOUGH_GRADIENT, HOUGH_GRADIENT_ALT, RETR_EXTERNAL,
        TM_CCOEFF_NORMED,
    },
    prelude::*,
};
use palette::{
    cast::{AsComponents, ComponentsAs},
    white_point::D65,
    IntoColor, Lab, Srgb, Srgba,
};
use ron::ser::{to_writer_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufWriter, path::PathBuf, process::exit};

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

    // let source2 = imread(&cli.path.to_string_lossy(), IMREAD_COLOR)?;
    // println!("{source2:?}");
    // let t = source2.to(CV_32F)?.reshape_def(1)?.clone_pointee();
    // println!("t: {t:?}");
    // let (compactness, labels) = t.kmeans(2, TermCriteria::default()?, 10, KMEANS_RANDOM_CENTERS)?;
    // println!("labels: {labels:?}");
    // labels
    //     .reshape(source2.channels(), source2.rows())?
    //     .write(cli.path.with_extension("labels.png"))?;

    let converge = 0.0025;
    // let converge = opt.factor.unwrap_or(if !opt.rgb { 5.0 } else { 0.0025 });

    let image = open(&cli.path)?.into_rgba8();
    let (width, height) = image.dimensions();
    // Convert image from Srgb to Lab
    let lab = image
        .as_raw()
        .components_as()
        .iter()
        .map(|&color| Srgba::<u8>::into_linear::<_, f32>(color).into_color())
        .collect::<Vec<Lab<D65, f32>>>();
    for k in 1..=config.kmeans.k {
        // Iterate over amount of runs keeping best results
        let mut kmeans = Kmeans::new();
        for index in 0..config.kmeans.runs {
            let r#try = get_kmeans_hamerly(
                k,
                config.kmeans.iterations,
                converge,
                false,
                &lab,
                config.kmeans.seed + index,
            );
            if r#try.score < kmeans.score {
                kmeans = r#try;
            }
        }
        // Convert centroids to Srgb<u8> before mapping to buffer
        let centroids = &kmeans
            .centroids
            .iter()
            .map(|&color| Srgb::from_linear(color.into_color()))
            .collect::<Vec<Srgb<u8>>>();
        let rgb: Vec<Srgb<u8>> = Srgb::map_indices_to_centroids(centroids, &kmeans.indices);
        // Write
        let writer = BufWriter::new(File::create(cli.path.with_extension(format!("{k}.png")))?);
        // If file is a palette, use Adaptive filtering to save more space
        let encoder =
            PngEncoder::new_with_quality(writer, CompressionType::Best, FilterType::NoFilter);
        // Clean up if file is created but there's a problem writing to it
        encoder.write_image(rgb.as_components(), width, height, ExtendedColorType::Rgb8)?;
    }

    let source = Mat::read(&cli.path, IMREAD_COLOR)?;
    let source2 = Mat::read(cli.path.with_extension("2.png"), IMREAD_COLOR)?;
    let source3 = Mat::read(cli.path.with_extension("3.png"), IMREAD_COLOR)?;

    // // Subtract
    // let subtracted = source2.subtract(&source1)?;
    // subtracted.write(cli.path.with_extension("subtracted.png"))?;

    // Gray
    let gray2 = source2.convert_color(COLOR_BGR2GRAY)?;
    gray2.write(cli.path.with_extension("gray2.png"))?;
    let gray3 = source3.convert_color(COLOR_BGR2GRAY)?;
    gray3.write(cli.path.with_extension("gray3.png"))?;

    // Template
    let template = Mat::read(&cli.path.with_file_name("template.10mum.png"), IMREAD_COLOR)?;
    let match_template = source.match_template(&template, TM_CCOEFF_NORMED, 0.9)?;
    let mut scale = source.clone();
    scale.draw_rectangle(match_template, RED)?;
    scale.write(cli.path.with_extension("scale.png"))?;
    let mean = source.mean(&template)?;
    println!("mean: {mean:?}");

    let mum = match_template.width / 10;
    println!("mum: {mum:?}");

    // Foreground
    let greater_than3 = gray3
        .greater_than(gray3.min(&no_array())?.1)?
        .bitwise_not()?;
    greater_than3.write(cli.path.with_extension("greater_than3.png"))?;
    // Background
    let less_than3 = gray3.less_than(gray3.max(&no_array())?.1)?;
    less_than3.write(cli.path.with_extension("less_than3.png"))?;
    // Unknown
    let subtract3 = less_than3.subtract(&greater_than3)?;
    subtract3.write(cli.path.with_extension("subtract3.png"))?;

    let mut target = source.clone();
    // Contours
    let contours = less_than3.find_contours(RETR_EXTERNAL, CHAIN_APPROX_SIMPLE)?;
    let size = less_than3.size()?;
    let mut filtered = Vector::<Mat>::default();
    for contour in &contours {
        if contour.area()? < config.contours.min_area {
            continue;
        }
        if contour.iter::<VecN<i32, 2>>()?.any(|(_, VecN([x, y]))| {
            x == 0 || y == 0 || x == size.width - 1 || y == size.height - 1
        }) {
            continue;
        }
        target.draw_contour(&contour, RED, 1)?;
        // convex hull
        let convex_hull = contour.convex_hull()?;
        target.draw_contour(&convex_hull, CYAN, 1)?;
        // rotated rectangle
        let rotated_rectangle = contour.rotated_rectangle()?;
        target.draw_rotated_rectangle(rotated_rectangle, GREEN)?;
        // max incircles
        let max_incircle = convex_hull.max_incircle()?;
        target.draw_circle(max_incircle.center, max_incircle.radius, BLUE, 1)?;
        let max_incircle = contour.max_incircle()?;
        target.draw_circle(max_incircle.center, max_incircle.radius, YELLOW, 1)?;
        // min circumcircle
        let min_circumcircle = contour.min_circumcircle()?;
        target.draw_circle(min_circumcircle.center, min_circumcircle.radius, MAGENTA, 1)?;
        filtered.push(contour);
    }
    target.write(cli.path.with_extension("target.png"))?;

    // // Threshold
    // let binary2 = gray2.threshold(0.0, 255.0, THRESH_BINARY_INV | THRESH_OTSU)?;
    // binary2.write(cli.path.with_extension("binary2.png"))?;
    // let binary3 = gray3.threshold(127.0, 255.0, ADAPTIVE_THRESH_GAUSSIAN_C)?;
    // binary3.write(cli.path.with_extension("binary3.png"))?;

    // Markers
    let mut markers = greater_than3.connected_components(8, CV_32S)?;
    markers.write(cli.path.with_extension("markers21.png"))?;
    for (index, (_, value)) in markers.iter_mut::<i32>()?.enumerate() {
        if *subtract3.at::<u8>(index as _)? == 255 {
            *value = 255;
        } else if *value != 0 {
            *value = 255;
        } else {
            *value = 0;
        }
    }
    markers.write(cli.path.with_extension("markers22.png"))?;

    // hough circles !!!!!!!!!!!!!!!!!!!!!!!!!!!
    let mut c = Mat::ones_size(source.size()?, CV_8U)?.to_mat()?;
    c.draw_contours(&filtered, WHITE, FILLED)?;
    c.write(cli.path.with_extension("_c.png"))?;
    let mut hough = source2.clone();
    let mut circles = Vector::<VecN<f32, 4>>::default();
    hough_circles(
        &c,
        &mut circles,
        HOUGH_GRADIENT_ALT,
        1.5,
        10.0,
        300.0,
        0.75,
        10,
        50,
    )?;
    for circle in circles {
        println!("circle: {circle:?}");
        hough.draw_circle(Point2f::new(circle[0], circle[1]), circle[2], RED, 1)?;
    }
    hough.write(cli.path.with_extension("_hough.png"))?;

    // source2.watershed(&mut markers)?;
    // markers.write(cli.path.with_extension("markers23.png"))?;

    // // Erode
    // let erode2 = markers.to(CV_32F)?.erode(
    //     &get_structuring_element_def(MORPH_ELLIPSE, Size::new(9, 9))?,
    //     1,
    // )?;
    // erode2.write(cli.path.with_extension("erode2.png"))?;
    // // Close
    // let close2 = markers.to(CV_32F)?.morphology(
    //     MORPH_OPEN,
    //     &get_structuring_element_def(MORPH_ELLIPSE, Size::new(3, 3))?,
    //     1,
    // )?;
    // close2.write(cli.path.with_extension("close2.png"))?;

    // let mut markers = temp3.connected_components(8, CV_32S)?;
    // markers.write(cli.path.with_extension("markers31.png"))?;
    // source3.watershed(&mut markers)?;
    // markers.write(cli.path.with_extension("markers32.png"))?;

    // // Distance transform
    // let distance_transform3 = less_than3.distance_transform(DIST_C, DIST_MASK_5)?;
    // distance_transform3.write(cli.path.with_extension("distance_transform3.png"))?;
    // let temp3 = distance_transform3
    //     .threshold(0.0, 255.0, ADAPTIVE_THRESH_MEAN_C)?
    //     .to(CV_8U)?;
    // temp3.write(cli.path.with_extension("temp3.png"))?;

    // // Remove noise
    // let opened = binary.morphology(MORPH_OPEN, &Mat::ones(3, 3, CV_8U)?, 3)?;
    // opened.write(cli.path.with_extension("opened.png"))?;

    // // Foreground
    // let distance_transform = opened.distance_transform(DIST_L2, DIST_MASK_5)?;
    // distance_transform.write(cli.path.with_extension("distance_transform.png"))?;

    // Canny
    let canny3 = gray3.canny(1.0, 0.0)?;
    canny3.write(cli.path.with_extension("canny3.png"))?;
    // Blur
    // let blur3 = canny3.median_blur(1)?;
    // blur3.write(cli.path.with_extension("blur3.png"))?;

    // // Dilate
    // let dilate3 = canny3.dilate(&Mat::ones(3, 3, CV_8U)?, 1)?;
    // dilate3.write(cli.path.with_extension("dilate3.png"))?;
    // // Erode
    // let erode3 = canny3.erode(&get_structuring_element_def(MORPH_CROSS, Size::new(7, 7))?)?;
    // erode3.write(cli.path.with_extension("erode3.png"))?;

    // let data = labels
    //     .iter::<i32>()?
    //     .map(|(_, value)| match value {
    //         0 => WHITE,
    //         value => color(value),
    //     })
    //     .collect::<Vec<_>>();
    // let colored_labels = Mat::from_slice(&data)?;
    // let m = colored_labels.reshape(4, rows)?;
    // m.write(cli.path.with_extension("colored_labels.png"))?;

    // for (_, value) in labels.iter_mut::<i32>()? {
    //     if *value == 0 {
    //         *value = 255;
    //     }
    // }
    // labels.write(cli.path.with_extension("labels.png"))?;

    // // Background
    // let background = binary.dilate(&Mat::ones(3, 3, CV_8U)?, 3)?;
    // background.write(cli.path.with_extension("background.png"))?;

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
