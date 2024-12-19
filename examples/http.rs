#[cfg(not(feature = "http"))]
compile_error!("This example requires the 'http' feature");

use cloudtiff::{CloudTiff, HttpReader};
use image::DynamicImage;
use ndarray;
use std::time::Instant;
use tokio;
// const URL: &str = "http://sentinel-cogs.s3.amazonaws.com/sentinel-s2-l2a-cogs/9/U/WA/2024/8/S2A_9UWA_20240806_0_L2A/TCI.tif";
const URL: &str = "http://localhost:5000/shared/test1.tif";
const OUTPUT_FILE: &str = "data/http_custom.jpg";
const PREVIEW_MEGAPIXELS: f64 = 1.0;

#[tokio::main]
async fn main() {
    println!("Example: cloudtiff async http");

    handler().await;
}

async fn handler() {
    // COG
    let t_cog = Instant::now();
    let mut http_reader = HttpReader::new(URL).unwrap();
    let cog = CloudTiff::open_async(&mut http_reader).await.unwrap();
    println!("Indexed COG in {}ms", t_cog.elapsed().as_millis());
    println!("{cog}");

    // Preview
    let t_preview = Instant::now();
    let preview = cog
        .renderer()
        .with_mp_limit(PREVIEW_MEGAPIXELS)
        .with_async_range_reader(http_reader)
        .render_async()
        .await
        .unwrap();
    println!(
        "Got preview in {:.6} seconds",
        t_preview.elapsed().as_secs_f64()
    );
    println!("{}", preview);

    // Image
    let img: DynamicImage = preview.clone().try_into().unwrap();
    let test = img.as_bytes().as_ref();
    let data = preview.to_f32_array().unwrap();

    // Now `data` is a Vec<f32> you can work with directly.
    let (height, width) = preview.dimensions;
    let array = ndarray::Array2::from_shape_vec((height as usize, width as usize), data)
        .expect("Failed to reshape data");
    let scaled: Vec<f32> = array.iter().map(|f| f.clamp(-1.0, 1.0)).collect();
    println!("{:?}", scaled);
    img.save(OUTPUT_FILE).unwrap();
    println!("Image saved to {OUTPUT_FILE}");
}
