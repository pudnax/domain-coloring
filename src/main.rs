extern crate image;
extern crate num_complex;

use num_complex::Complex;

mod col;
use col::ColormapType;

const W: u32 = 2560;
const H: u32 = 1600;
const FOVY: u32 = 1;
const SUPER_SAMPLING: u32 = 1;

static SW: u32 = SUPER_SAMPLING * W;
static SH: u32 = SUPER_SAMPLING * H;

fn complex_function(z: Complex<f64>) -> Complex<f64> {
    // z.inv().sin()
    ((z - 1.) / (z + 1.)).cos().powi(20)
}

fn complex_color(z: Complex<f64>, color_map: &col::ColorMap) -> image::Rgba<u8> {
    let phase = z.arg();
    let mut t = phase / std::f64::consts::PI + 1.;
    if t > 1. {
        t = 2. - t;
    }

    let c = color_map.at(t);
    image::Rgba([c.r(), c.g(), c.b(), 255])
}

fn pixel_coordinates(px: u32, py: u32) -> (f64, f64) {
    let center_real = 0.;
    let center_imag = 0.;
    let sw_f = f64::from(SW);
    let sh_f = f64::from(SH);
    let aspect_ratio = f64::from(W) / f64::from(H);
    let half_fovy = f64::from(FOVY) / 2.;

    let x = ((f64::from(px) / (sw_f - 1.)) * 2. - 1.) * aspect_ratio * half_fovy + center_real;
    let y = (((sh_f - f64::from(py) - 1.) / (sh_f - 1.)) * 2. - 1.) * half_fovy + center_imag;
    (x, y)
}

fn main() {
    let mut imgbuf = image::ImageBuffer::new(SW, SH);

    let (x0, y0) = pixel_coordinates(0, 0);
    let (x1, y1) = pixel_coordinates(SW - 1, SH - 1);
    let dx = (x1 - x0) / f64::from(SW - 1);
    let dy = (y1 - y0) / f64::from(SH - 1);

    let color_map = col::ColorMap::new(ColormapType::Inferno);

    let mut y = y0;
    for py in 0..SH {
        let mut x = x0;
        for px in 0..SW {
            let z = complex_function(Complex::new(x, y));
            let c = complex_color(z, &color_map);
            imgbuf.put_pixel(px, py, c);
            x += dx;
        }
        y += dy;
    }

    imgbuf.save("pic.png").unwrap();
}
