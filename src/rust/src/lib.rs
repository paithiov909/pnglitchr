pub use crate::operation::Transpose;
use crate::operation::{Encode, Scan};
use crate::png::Png;
pub use crate::png::{FilterType, ScanLine};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::vec;

use rand::Rng;
use savvy::{savvy, savvy_err};

mod operation;
mod png;

/// PngGlitch is a crate to create a glitched PNG image.
/// Please refer to ["The Art of PNG glitch"](https://ucnv.github.io/pnglitch/) for the description about what glitched PNG is.
///
/// # Examples
///
/// The following snippet shows how you can glitch "./etc/sample00.png" and save the generated image as "./glitched.png".
///
/// ```
/// # use std::env;
/// # env::set_current_dir(env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string())).expect("");
///
/// use png_glitch::{FilterType, PngGlitch};
///
/// let mut png_glitch = PngGlitch::open("./etc/sample00.png").expect("The PNG file should be successfully parsed");
/// png_glitch.foreach_scanline(|scan_line|{
///   scan_line.set_filter_type(FilterType::None);
///   let pixel = scan_line.index(4).unwrap_or(0);
///   scan_line.update(4, pixel / 2);
/// });
/// png_glitch.save("./glitched.png").expect("The glitched file should be saved as a PNG file");
/// ```
///
pub struct PngGlitch {
    png: Png,
}

impl PngGlitch {
    /// The method creates a PngGlitch object to glitch the PNG image loaded from the given file path.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::env;
    /// # env::set_current_dir(env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string())).expect("");
    ///
    /// use png_glitch::PngGlitch;
    ///
    /// let mut png_glitch = PngGlitch::open("./etc/sample00.png").expect("The PNG file should be successfully parsed");
    /// ```
    pub fn open(path: impl AsRef<Path>) -> anyhow::Result<PngGlitch> {
        let mut file = File::open(path)?;
        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        PngGlitch::new(buf)
    }

    /// The method creates a PngGlitch object to glitch the PNG image stored in a given `Vec<u8>`.
    ///
    /// # Example
    ///
    /// A PngGlitch object is created from a `Vec<u8>` object containing PNG image data in the following snippet.
    ///
    /// ```
    /// use std::fs::File;
    /// use std::io::Read;
    /// use png_glitch::PngGlitch;
    ///
    /// let mut buffer = vec![];
    /// let mut file = File::open("./etc/sample00.png").expect("The file should be opened");
    /// file.read_to_end(&mut buffer).expect("The bytes in the file should be written into the buffer");
    /// let mut png_glitch = PngGlitch::new(buffer).expect("The data in the buffer should be successfully parsed as PNG");
    /// ```
    pub fn new(buffer: Vec<u8>) -> anyhow::Result<PngGlitch> {
        let png = Png::try_from(&buffer as &[u8])?;
        Ok(PngGlitch { png })
    }

    /// The method returns a list of [scan line](https://www.w3.org/TR/2003/REC-PNG-20031110/#4Concepts.EncodingScanlineAbs%22). in the given PNG file.
    ///
    /// # Example
    ///
    /// The following example changes the filter type of each scan line according its position
    ///
    /// ```
    /// # use std::env;
    /// # env::set_current_dir(env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string())).expect("");
    /// use png_glitch::{FilterType, PngGlitch};
    ///
    /// let mut png_glitch = PngGlitch::open("./etc/sample00.png").expect("The PNG file should be successfully parsed");
    /// for (index, scan_line) in png_glitch.scan_lines().iter_mut().enumerate() {
    ///    let filter_type = if index % 2 == 0 {
    ///        FilterType::None
    ///    } else {
    ///        FilterType::Average
    ///    };
    ///    scan_line.set_filter_type(filter_type);
    /// }
    /// ```
    pub fn scan_lines(&self) -> Vec<ScanLine> {
        self.png.scan_lines()
    }

    /// The method takes the specified number of ScanLine objects at most.
    /// The maximum number of ScanLines is specified as `lines` parameter.
    /// The `from` parameter specifies the index of first ScanLine.
    ///
    /// # Example
    /// ```
    /// use png_glitch::{FilterType, PngGlitch};
    ///
    /// let mut png_glitch = PngGlitch::open("./etc/sample00.png").expect("The PNG file should be successfully parsed");
    /// let scan_liens = png_glitch.scan_lines_from(5, 10);
    /// ```
    pub fn scan_lines_from(&self, from: u32, lines: u32) -> Vec<ScanLine> {
        self.png.scan_lines_from(from as usize, lines as usize)
    }

    /// The method allows you to manipulate for each [scan line](https://www.w3.org/TR/2003/REC-PNG-20031110/#4Concepts.EncodingScanlineAbs%22).
    /// The modifier function is called with a `ScanLine` object which represents a scan line.
    ///
    /// # Example
    ///
    /// The following example changes the filter method of all scan line to None.
    ///
    /// ```
    /// # use std::env;
    /// # env::set_current_dir(env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string())).expect("");
    ///
    /// use png_glitch::{FilterType, PngGlitch};
    ///
    /// let mut png_glitch = PngGlitch::open("./etc/sample00.png").expect("The PNG file should be successfully parsed");
    /// png_glitch.foreach_scanline(|scan_line|{
    ///    scan_line.set_filter_type(FilterType::None);
    /// });
    /// ```
    pub fn foreach_scanline<F>(&self, modifier: F)
    where
        F: FnMut(&mut ScanLine),
    {
        self.png.foreach_scanline(modifier)
    }

    /// The method saves the glitched image as a PNG file to the given path.
    ///
    /// # Example
    ///
    /// The following example copies `./etc/sample00.png` as `./glitched.png`.
    /// ```
    /// use png_glitch::{FilterType, PngGlitch};
    ///
    /// let png_glitch = PngGlitch::open("etc/sample00.png").expect("The PNG file should be successfully parsed");
    /// png_glitch.save("./glitched.png").expect("The glitched PNG data should be saved to the given path");
    /// ```
    pub fn save(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        self.png.save(path)
    }

    /// The method encodes the glitched image as a PNG data and write the encoded data to the given buffer.
    ///
    /// # Example
    ///
    /// The following example writes a PNG format data into the `encoded_data`.
    ///
    /// ```
    /// # use std::env;
    /// # env::set_current_dir(env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string())).expect("");
    /// use png_glitch::PngGlitch;
    ///
    /// let png_glitch = PngGlitch::open("./etc/sample00.png").expect("The PNG file should be successfully parsed");
    /// let mut encoded_data:Vec<u8> = vec![];
    /// png_glitch.encode(&mut encoded_data).expect("The glitched PNG data should be written into the encoded_data in PNG format");
    /// ```
    pub fn encode(&self, buffer: &mut Vec<u8>) -> anyhow::Result<()> {
        self.png.encode(buffer)?;
        Ok(())
    }

    /// The method returns the width of the loaded PNG file
    ///
    /// # Example
    ///
    /// The following example retrieves width of ./etc/sample00.png
    ///
    /// ```
    /// # use std::env;
    /// # env::set_current_dir(env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string())).expect("");
    /// use png_glitch::PngGlitch;
    ///
    /// let png_glitch = PngGlitch::open("./etc/sample00.png").expect("The PNG file should be successfully parsed");
    /// let width = png_glitch.width();
    /// ```
    pub fn width(&self) -> u32 {
        self.png.width()
    }

    /// The method returns the height of the loaded PNG file
    ///
    /// # Example
    ///
    /// The following example retrieves height of ./etc/sample00.png
    ///
    /// ```
    /// # use std::env;
    /// # env::set_current_dir(env::var("CARGO_MANIFEST_DIR").unwrap_or(".".to_string())).expect("");
    /// use png_glitch::PngGlitch;
    ///
    /// let png_glitch = PngGlitch::open("./etc/sample00.png").expect("The PNG file should be successfully parsed");
    /// let width = png_glitch.width();
    /// ```
    pub fn height(&self) -> u32 {
        self.png.height()
    }

    /// The method copies the lines starting from src to dest
    ///
    /// # Example
    ///
    /// ```
    /// use png_glitch::PngGlitch;
    /// let mut png_glitch = PngGlitch::open("./etc/sample00.png").expect("The PNG file should be successfully parsed");
    /// let width = png_glitch.transpose(2, 5, 10);
    /// ```
    pub fn transpose(&mut self, src: u32, dst: u32, lines: u32) {
        self.png.transpose(src as usize, dst as usize, lines)
    }

    /// The method removes filter from all scan lines.
    ///
    /// # Example
    ///
    /// ```
    /// use png_glitch::PngGlitch;
    /// let mut png_glitch = PngGlitch::open("./etc/sample00.png").expect("The PNG file should be successfully parsed");
    /// png_glitch.remove_filter();
    /// png_glitch.save("./etc/removed-all.png").expect("The PNG file should be successfully saved")
    /// ```
    pub fn remove_filter(&mut self) {
        self.png.remove_filter();
    }

    /// The method removes filter from the scan lines in specified region
    ///
    /// # Example
    ///
    /// ```
    /// use png_glitch::PngGlitch;
    /// let mut png_glitch = PngGlitch::open("./etc/sample00.png").expect("The PNG file should be successfully parsed");
    /// png_glitch.remove_filter_from(5, 10); // Remove filter from the scan line #5 - # 14
    /// png_glitch.save("./etc/removed-partial.png").expect("The PNG file should be successfully saved")
    /// ```
    pub fn remove_filter_from(&mut self, from: u32, lines: u32) {
        self.png.remove_filter_from(from, lines);
    }

    /// The method removes filter from all scan lines.
    ///
    /// # Example
    ///
    /// ```
    /// use png_glitch::{FilterType, PngGlitch};
    /// let mut png_glitch = PngGlitch::open("./etc/none.png").expect("The PNG file should be successfully parsed");
    /// png_glitch.apply_filter(FilterType::Sub);
    /// png_glitch.save("./etc/filter-all.png").expect("The PNG file should be successfully saved")
    /// ```
    pub fn apply_filter(&mut self, filter: FilterType) {
        self.png.apply_filter(filter);
    }

    /// The method removes filter from scan lines in specified region
    ///
    /// # Example
    ///
    /// ```
    /// use png_glitch::{FilterType, PngGlitch};
    /// let mut png_glitch = PngGlitch::open("./etc/none.png").expect("The PNG file should be successfully parsed");
    /// png_glitch.apply_filter_from(FilterType::Sub, 5, 3); // Apply sub filter to the scan line #5, #6, and #7.
    /// png_glitch.save("./etc/filter-partial.png").expect("The PNG file should be successfully saved")
    /// ```
    pub fn apply_filter_from(&mut self, filter_type: FilterType, from: u32, lines: u32) {
        self.png.apply_filter_from(filter_type, from, lines);
    }
}

fn raw_to_png_glitch(raw: savvy::RawSexp) -> anyhow::Result<PngGlitch> {
    let raw = raw.to_vec();
    PngGlitch::new(raw)
}

/// Random copy
///
/// @param bytes PNG image data
/// @param times Number of times to copy
/// @returns PNG image data
/// @noRd
#[savvy]
fn pgltc_random_copy(bytes: savvy::RawSexp, times: savvy::NumericScalar) -> savvy::Result<savvy::Sexp> {
    let png = raw_to_png_glitch(bytes).map_err(|_| savvy_err!("Failed to parse PNG data"))?;

    let mut scan_lines = png.scan_lines();
    let mut rng = rand::thread_rng();
    let index_range = 0..scan_lines.len();
    let times = times.as_i32()?;

    for _ in 0..times {
        let src = rng.gen_range(index_range.clone());
        let dst = rng.gen_range(index_range.clone());

        let src = &mut scan_lines[src];
        let filter_type = src.filter_type();
        let mut buffer = vec![];
        src.read_to_end(&mut buffer)
            .map_err(|_| savvy_err!("Failed to read scan line"))?;

        let dst = &mut scan_lines[dst];
        dst.write_all(&buffer)
            .map_err(|_| savvy_err!("Failed to write scan line"))?;
        dst.set_filter_type(filter_type);
    }
    let mut buf = vec![];
    png.encode(&mut buf)
        .map_err(|_| savvy_err!("Failed to encode PNG data"))?;

    let out = savvy::OwnedRawSexp::try_from(buf)?;
    Ok(out.into())
}

/// Remove filters
///
/// @param bytes PNG image data
/// @param from Scan line index
/// @param lines Number of scan lines to be updated
/// @returns PNG image data
/// @noRd
#[savvy]
fn pgltc_remove_filter(bytes: savvy::RawSexp, from: savvy::NumericScalar, lines: savvy::NumericScalar) -> savvy::Result<savvy::Sexp> {
    let mut png = raw_to_png_glitch(bytes).map_err(|_| savvy_err!("Failed to parse PNG data"))?;
    let from = from.as_i32()? as u32;
    let lines = lines.as_i32()? as u32;

    png.remove_filter_from(from, lines);
    let mut buf = vec![];
    png.encode(&mut buf)
        .map_err(|_| savvy_err!("Failed to encode PNG data"))?;

    let out = savvy::OwnedRawSexp::try_from(buf)?;
    Ok(out.into())
}

/// Transpose
///
/// @param bytes PNG image data
/// @param src Scan line index
/// @param dst Scan line index
/// @param lines Number of scan lines to be updated
/// @returns PNG image data
/// @noRd
#[savvy]
fn pgltc_transpose(bytes: savvy::RawSexp, src: savvy::NumericScalar, dst: savvy::NumericScalar, lines: savvy::NumericScalar) -> savvy::Result<savvy::Sexp> {
    let mut png = raw_to_png_glitch(bytes).map_err(|_| savvy_err!("Failed to parse PNG data"))?;
    let src = src.as_i32()? as u32;
    let dst = dst.as_i32()? as u32;
    let lines = lines.as_i32()? as u32;

    png.transpose(src, dst, lines);
    let mut buf = vec![];
    png.encode(&mut buf)
        .map_err(|_| savvy_err!("Failed to encode PNG data"))?;

    let out = savvy::OwnedRawSexp::try_from(buf)?;
    Ok(out.into())
}

/// Apply filter
///
/// @param bytes PNG image data
/// @param filter_type Filter type. 0: None, 1: Sub, 2: Up, 3: Average, 4: Paeth
/// @param from Scan line index
/// @param lines Number of scan lines to be updated
/// @returns PNG image data
/// @noRd
#[savvy]
fn pgltc_apply_filter(bytes: savvy::RawSexp, filter_type: savvy::NumericScalar, from: savvy::NumericScalar, lines: savvy::NumericScalar) -> savvy::Result<savvy::Sexp> {
    let mut png = raw_to_png_glitch(bytes).map_err(|_| savvy_err!("Failed to parse PNG data"))?;
    let filter_type = match filter_type.as_i32()? {
        0 => FilterType::None,
        1 => FilterType::Sub,
        2 => FilterType::Up,
        3 => FilterType::Average,
        4 => FilterType::Paeth,
        _ => FilterType::None,
    };
    let from = from.as_i32()? as u32;
    let lines = lines.as_i32()? as u32;

    png.apply_filter_from(filter_type, from, lines);
    let mut buf = vec![];
    png.encode(&mut buf)
        .map_err(|_| savvy_err!("Failed to encode PNG data"))?;

    let out = savvy::OwnedRawSexp::try_from(buf)?;
    Ok(out.into())
}

/// Count scanlines
///
/// @param bytes PNG image data
/// @returns Total number of scanlines
/// @noRd
#[savvy]
fn pgltc_count_scanlines(bytes: savvy::RawSexp) -> savvy::Result<savvy::Sexp> {
    let png = raw_to_png_glitch(bytes).map_err(|_| savvy_err!("Failed to parse PNG data"))?;
    let n = png.scan_lines().len() as i32;
    let out = savvy::OwnedIntegerSexp::try_from_scalar(n)?;
    Ok(out.into())
}
