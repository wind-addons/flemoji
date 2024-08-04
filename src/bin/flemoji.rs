use clap::{Arg, Command};
use image::ImageFormat;
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
struct CustomOptions {
    input_dir: String,
    output_dir: String,
    width: u32,
    height: u32,
    file_type: ImageFormat,
}

fn parse_args() -> CustomOptions {
    let matches = Command::new("flemoji")
        .version(env!("CARGO_PKG_VERSION"))
        .about("flemoji - Customize Microsoft Fluent UI emoji images.")
        .author("fang2hou")
        .arg(
            Arg::new("width")
                .short('W')
                .long("width")
                .value_name("WIDTH")
                .help("Sets the width of the output images")
                .required(true),
        )
        .arg(
            Arg::new("height")
                .short('H')
                .long("height")
                .value_name("HEIGHT")
                .help("Sets the height of the output images")
                .required(true),
        )
        .arg(
            Arg::new("from")
                .long("from")
                .value_name("DIR")
                .help("Sets the input directory (assets)")
                .required(true),
        )
        .arg(
            Arg::new("to")
                .long("to")
                .value_name("DIR")
                .help("Sets the output directory")
                .required(true),
        )
        .arg(
            Arg::new("filetype")
                .short('T')
                .long("filetype")
                .value_name("FILETYPE")
                .help("Sets the output file type (png, jpg, etc.)")
                .default_value("png"),
        )
        .get_matches();

    CustomOptions {
        input_dir: matches.get_one::<String>("from").unwrap().clone(),
        output_dir: matches.get_one::<String>("to").unwrap().clone(),
        width: matches.get_one::<String>("width").unwrap().parse().unwrap(),
        height: matches
            .get_one::<String>("height")
            .unwrap()
            .parse()
            .unwrap(),
        file_type: match matches.get_one::<String>("filetype").unwrap().as_str() {
            "png" => ImageFormat::Png,
            "jpg" | "jpeg" => ImageFormat::Jpeg,
            "gif" => ImageFormat::Gif,
            "bmp" => ImageFormat::Bmp,
            "ico" => ImageFormat::Ico,
            "tga" => ImageFormat::Tga,
            _ => panic!("Unsupported file type"),
        },
    }
}

fn resize_and_save_image(
    input_path: &Path,
    output_dir: &str,
    width: u32,
    height: u32,
    format: ImageFormat,
) -> Result<(), Box<dyn std::error::Error>> {
    let img = image::open(input_path)?;
    let img = img.resize(width, height, image::imageops::Lanczos3);
    let output_path = Path::new(output_dir).join(
        input_path
            .file_name()
            .ok_or("Invalid file name")?
            .to_str()
            .ok_or("Invalid file name")?
            .replace(
                &format!(".{}", input_path.extension().unwrap().to_str().unwrap()),
                &format!(".{}", format.extensions_str()[0]),
            ),
    );
    img.save_with_format(&output_path, format)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args();

    fs::create_dir_all(&args.output_dir)?;

    WalkDir::new(&args.input_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .filter(|e| e.path().extension().and_then(|e| e.to_str()) == Some("png"))
        .filter(|e| {
            e.path()
                .parent()
                .and_then(|p| p.to_str())
                .map_or(false, |s| s.contains("3D"))
        })
        .par_bridge()
        .for_each(|entry| {
            match resize_and_save_image(
                entry.path(),
                &args.output_dir,
                args.width,
                args.height,
                args.file_type,
            ) {
                Ok(_) => println!("Processed: {}", entry.path().display()),
                Err(e) => eprintln!("Error processing {}: {}", entry.path().display(), e),
            }
        });

    println!(
        "Conversion complete. Resized files saved in {}",
        args.output_dir
    );
    Ok(())
}
