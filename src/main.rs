use clap::Parser;
use pdfium_render::prelude::*;
use std::fs;
use std::path::PathBuf;


//define ClI tools to convert odf images to images

#[derive(Parser, Debug)]
#[command(name = "pdf2image")]
#[command(about = "Convert PDF to images using pdfium-render", long_about = None)]
struct Args {

    /// Input pdf file PathBuf
    #[arg(short, long)]
    input: PathBuf,

    //output dir
    #[arg(short, long, default_value="./images")]
    output_dir: PathBuf,

    /// Image format png of jpg
    #[arg(short, long, default_value="png")]
    format: String,

    /// DPI resolution for rendering
    #[arg(short, long, default_value_t=150)]
    dpi: u16,

    #[arg(short, long, default_value=None)]
    password: Option<String>

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    fs::create_dir_all(&args.output_dir)?;

    // Load Pdfium (uses the bundled binary if available)
    let pdfium = Pdfium::default();

    // Load the PDF document
    let doc = pdfium.load_pdf_from_file(&args.input, args.password.as_deref())?;

    println!("✅ PDF started converted to images in {:?}", args.output_dir);

    // Render each page
    for (index, page) in doc.pages().iter().enumerate() {
        let render = page.render_with_config(
            &PdfRenderConfig::new()
                .set_target_width((args.dpi as f32 * 8.5) as i32)
                .set_target_height((args.dpi as f32 * 11.0) as i32)
                .render_form_data(true),
        );

        let image = render?.as_image();

        let pdf_name = args
            .input
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();

        let filename = format!("{}_page_{}.{}", pdf_name, index + 1, args.format);

        let out_path = args.output_dir.join(filename);

        match args.format.as_str() {
            "png" => image.save(out_path)?,
            "jpg" | "jpeg" => image.save_with_format(out_path, image::ImageFormat::Jpeg)?,
            _ => {
                eprintln!("Unsupported image format: {}", args.format);
                std::process::exit(1);
            }
        }
    }

    println!("✅ PDF successfully converted to images in {:?}", args.output_dir);

    Ok(())

}
