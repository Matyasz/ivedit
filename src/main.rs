use argh::FromArgs;
use image::io::Reader as ImageReader;

#[derive(FromArgs, Debug)]
#[argh(description = "CLI for the Image and Video EDITor")]
struct Cli {
    #[argh(option, short = 'f', description = "the file to edit")]
    file: String,

    #[argh(option, short = 'x', default = "0", description = "width of the output")]
    width: u32,

    #[argh(option, short = 'y', default = "0", description = "height of the output")]
    height: u32,
}
fn main() {
    let args: Cli = argh::from_env();

    dbg!(&args);

    let mut img = ImageReader::open(&args.file).unwrap().decode().unwrap();
    dbg!(img.width());
    dbg!(img.height());

    img = img.resize_exact(args.width, args.height, image::imageops::FilterType::Gaussian);
    dbg!(img.width());
    dbg!(img.height());

    img.save("./outputs/1.jpg").unwrap();
}