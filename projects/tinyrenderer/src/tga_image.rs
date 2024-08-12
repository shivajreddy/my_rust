pub struct TGAHeader {
    pub idlength: u8,
    pub colormaptype: u8,
    pub datatypecode: u8,
    pub colormaporigin: i16,
    pub colormaplength: i16,
    pub colormapdepth: u8,
    pub x_origin: i16,
    pub y_origin: i16,
    pub width: i16,
    pub height: i16,
    pub bitsperpixel: u8,
    pub imagedescriptor: u8,
}

pub struct TGAColor {
    r: u8,
    g: u8,
    a: u8,
    b: u8,
    bytespp: usize,
}

impl TGAColor {
    // Default Constructor
    pub fn new() -> Self {
        TGAColor {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
            bytespp: 1,
        }
    }

    // Constructor with RGBA values
    pub fn with_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        TGAColor {
            r,
            g,
            b,
            a,
            bytespp: 4,
        }
    }

    // Constructor with a single integer value and bytes per pixel
    pub fn with_val(val: u32, bytespp: usize) -> Self {
        let r = ((val >> 16) & 0xFF) as u8;
        let g = ((val >> 8) & 0xFF) as u8;
        let b = (val & 0xFF) as u8;
        let a = ((val >> 24) & 0xFF) as u8;
        TGAColor {
            r,
            g,
            b,
            a,
            bytespp,
        }
    }

    // Constructor with a slice of bytes and bytes per pixel
    pub fn with_slice(p: &[u8], bytespp: usize) -> Self {
        let mut color = TGAColor::new();
        color.bytespp = bytespp;
        for i in 0..bytespp {
            match i {
                0 => color.b = p[i],
                1 => color.g = p[i],
                2 => color.r = p[i],
                3 => color.a = p[i],
                _ => (),
            }
        }
        color
    }
}

#[derive(Clone, Debug)]
struct TGAImage {
    data: Vec<u8>, // this is `unsigned char *` in C++
    width: i32,
    height: i32,
    bytespp: i32,
}

pub enum Format {
    Grayscale = 1,
    RGB = 3,
    RGBA = 4,
}

impl TGAImage {
    pub fn new() -> Self {
        TGAImage {
            data: Vec::new(),
            width: 0,
            height: 0,
            bytespp: 0,
        }
    }

    pub fn with_dimensions(w: i32, h: i32, bpp: i32) -> Self {
        TGAImage {
            data: vec![0; (w * h * bpp) as usize],
            width: w,
            height: h,
            bytespp: bpp,
        }
    }

    pub fn read_tga_file<P: AsRef<Path>>(&mut self, filename: P) -> io::Result<bool> {
        let mut file = File::open(filename)?;
        let mut header = [0u8; 18];
        file.read_exact(&mut header)?;

        self.width = (header[12] as i32) | ((header[13] as i32) << 8);
        self.height = (header[14] as i32) | ((header[15] as i32) << 8);
        self.bytespp = (header[16] / 8) as i32;
        let nbytes = (self.width * self.height * self.bytespp) as usize;
        self.data = vec![0; nbytes];

        file.read_exact(&mut self.data)?;
        Ok(true)
    }

    pub fn write_tga_file<P: AsRef<Path>>(&self, filename: P, rle: bool) -> io::Result<bool> {
        let mut file = File::create(filename)?;
        let mut header = [0u8; 18];
        header[2] = if rle { 10 } else { 2 }; // Uncompressed or RLE compressed
        header[12] = (self.width & 0xFF) as u8;
        header[13] = ((self.width >> 8) & 0xFF) as u8;
        header[14] = (self.height & 0xFF) as u8;
        header[15] = ((self.height >> 8) & 0xFF) as u8;
        header[16] = (self.bytespp * 8) as u8;
        file.write_all(&header)?;

        file.write_all(&self.data)?;
        Ok(true)
    }

    pub fn flip_horizontally(&mut self) -> bool {
        // Implementation for horizontal flip
        true
    }

    pub fn flip_vertically(&mut self) -> bool {
        // Implementation for vertical flip
        true
    }

    pub fn scale(&mut self, w: i32, h: i32) -> bool {
        // Implementation for scaling
        true
    }

    pub fn get(&self, x: i32, y: i32) -> Option<TGAColor> {
        if x >= self.width || y >= self.height || x < 0 || y < 0 {
            return None;
        }
        let index = ((y * self.width + x) * self.bytespp) as usize;
        Some(TGAColor::with_slice(
            &self.data[index..index + self.bytespp as usize],
            self.bytespp,
        ))
    }

    pub fn set(&mut self, x: i32, y: i32, c: TGAColor) -> bool {
        if x >= self.width || y >= self.height || x < 0 || y < 0 {
            return false;
        }
        let index = ((y * self.width + x) * self.bytespp) as usize;
        let color_data = c.as_bytes();
        for i in 0..self.bytespp as usize {
            self.data[index + i] = color_data[i];
        }
        true
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn get_bytespp(&self) -> i32 {
        self.bytespp
    }

    pub fn buffer(&self) -> &[u8] {
        &self.data
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.width = 0;
        self.height = 0;
        self.bytespp = 0;
    }

    // Helper methods for RLE (not fully implemented)
    fn load_rle_data(&mut self, _in: &mut File) -> bool {
        // Implementation for loading RLE data
        true
    }

    fn unload_rle_data(&self, _out: &mut File) -> bool {
        // Implementation for unloading RLE data
        true
    }
}
