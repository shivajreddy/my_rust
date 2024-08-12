use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::mem;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct TGAImage {
    data: Vec<u8>,
    width: i32,
    height: i32,
    bytespp: i32,
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
        let nbytes = (w * h * bpp) as usize;
        TGAImage {
            data: vec![0; nbytes],
            width: w,
            height: h,
            bytespp: bpp,
        }
    }

    pub fn from_image(img: &TGAImage) -> Self {
        TGAImage {
            width: img.width,
            height: img.height,
            bytespp: img.bytespp,
            data: img.data.clone(),
        }
    }

    pub fn read_tga_file<P: AsRef<Path>>(&mut self, filename: P) -> io::Result<bool> {
        let mut file = BufReader::new(File::open(filename)?);

        let mut header = TGAHeader::default();
        file.read_exact(header.as_mut())?;

        self.width = header.width as i32;
        self.height = header.height as i32;
        self.bytespp = (header.bitsperpixel >> 3) as i32;

        if self.width <= 0 || self.height <= 0 || !matches!(self.bytespp, 1 | 3 | 4) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid TGA file dimensions or format",
            ));
        }

        let nbytes = (self.bytespp * self.width * self.height) as usize;
        self.data = vec![0; nbytes];

        match header.datatypecode {
            2 | 3 => file.read_exact(&mut self.data)?,
            10 | 11 => self.load_rle_data(&mut file)?,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unknown file format",
                ))
            }
        };

        if header.imagedescriptor & 0x20 == 0 {
            self.flip_vertically();
        }
        if header.imagedescriptor & 0x10 != 0 {
            self.flip_horizontally();
        }

        Ok(true)
    }

    fn load_rle_data<R: Read>(&mut self, file: &mut R) -> io::Result<bool> {
        let pixelcount = (self.width * self.height) as usize;
        let mut currentpixel = 0;
        let mut currentbyte = 0;
        let mut colorbuffer = TGAColor::default();

        while currentpixel < pixelcount {
            let chunkheader = file.bytes().next().ok_or_else(|| {
                io::Error::new(io::ErrorKind::UnexpectedEof, "Unexpected end of file")
            })??;

            if chunkheader < 128 {
                for _ in 0..chunkheader + 1 {
                    file.read_exact(colorbuffer.as_mut_slice(self.bytespp as usize))?;
                    self.data[currentbyte..currentbyte + self.bytespp as usize]
                        .copy_from_slice(colorbuffer.as_slice(self.bytespp as usize));
                    currentbyte += self.bytespp as usize;
                    currentpixel += 1;
                }
            } else {
                file.read_exact(colorbuffer.as_mut_slice(self.bytespp as usize))?;
                for _ in 0..(chunkheader - 127) {
                    self.data[currentbyte..currentbyte + self.bytespp as usize]
                        .copy_from_slice(colorbuffer.as_slice(self.bytespp as usize));
                    currentbyte += self.bytespp as usize;
                    currentpixel += 1;
                }
            }
        }

        Ok(true)
    }

    pub fn write_tga_file<P: AsRef<Path>>(&self, filename: P, rle: bool) -> io::Result<bool> {
        let mut file = BufWriter::new(File::create(filename)?);

        let mut header = TGAHeader::default();
        header.width = self.width as u16;
        header.height = self.height as u16;
        header.bitsperpixel = (self.bytespp * 8) as u8;
        header.datatypecode = if rle {
            if self.bytespp == 1 {
                11
            } else {
                10
            }
        } else {
            if self.bytespp == 1 {
                3
            } else {
                2
            }
        };
        header.imagedescriptor = 0x20;

        file.write_all(header.as_ref())?;

        if !rle {
            file.write_all(&self.data)?;
        } else {
            self.unload_rle_data(&mut file)?;
        }

        let developer_area_ref = [0u8; 4];
        let extension_area_ref = [0u8; 4];
        let footer = b"TRUEVISION-XFILE.\0";

        file.write_all(&developer_area_ref)?;
        file.write_all(&extension_area_ref)?;
        file.write_all(footer)?;

        Ok(true)
    }

    fn unload_rle_data<W: Write>(&self, out: &mut W) -> io::Result<bool> {
        let max_chunk_length = 128;
        let npixels = (self.width * self.height) as usize;
        let mut curpix = 0;

        while curpix < npixels {
            let chunkstart = curpix * self.bytespp as usize;
            let mut curbyte = curpix * self.bytespp as usize;
            let mut run_length = 1;
            let mut raw = true;

            while curpix + run_length < npixels && run_length < max_chunk_length {
                let mut succ_eq = true;
                for t in 0..self.bytespp as usize {
                    succ_eq =
                        self.data[curbyte + t] == self.data[curbyte + t + self.bytespp as usize];
                    if !succ_eq {
                        break;
                    }
                }

                curbyte += self.bytespp as usize;

                if run_length == 1 {
                    raw = !succ_eq;
                }

                if raw && succ_eq {
                    run_length -= 1;
                    break;
                }

                if !raw && !succ_eq {
                    break;
                }

                run_length += 1;
            }

            curpix += run_length;
            out.write_all(&[if raw {
                run_length - 1
            } else {
                run_length + 127
            }])?;

            out.write_all(
                &self.data[chunkstart
                    ..chunkstart
                        + if raw {
                            run_length * self.bytespp as usize
                        } else {
                            self.bytespp as usize
                        }],
            )?;
        }

        Ok(true)
    }

    pub fn get(&self, x: i32, y: i32) -> Option<TGAColor> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }
        let index = ((x + y * self.width) * self.bytespp) as usize;
        Some(TGAColor::from_slice(
            &self.data[index..index + self.bytespp as usize],
            self.bytespp,
        ))
    }

    pub fn set(&mut self, x: i32, y: i32, c: &TGAColor) -> bool {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return false;
        }
        let index = ((x + y * self.width) * self.bytespp) as usize;
        self.data[index..index + self.bytespp as usize]
            .copy_from_slice(c.as_slice(self.bytespp as usize));
        true
    }

    pub fn flip_horizontally(&mut self) -> bool {
        if self.data.is_empty() {
            return false;
        }
        let half = self.width / 2;
        for i in 0..half {
            for j in 0..self.height {
                let c1 = self.get(i, j).unwrap();
                let c2 = self.get(self.width - 1 - i, j).unwrap();
                self.set(i, j, &c2);
                self.set(self.width - 1 - i, j, &c1);
            }
        }
        true
    }

    pub fn flip_vertically(&mut self) -> bool {
        if self.data.is_empty() {
            return false;
        }
        let bytes_per_line = (self.width * self.bytespp) as usize;
        let half = self.height / 2;
        for j in 0..half {
            let l1 = j as usize * bytes_per_line;
            let l2 = (self.height as usize - 1 - j as usize) * bytes_per_line;
            self.data[l1..l1 + bytes_per_line]
                .swap_with_slice(&mut self.data[l2..l2 + bytes_per_line]);
        }
        true
    }

    pub fn scale(&mut self, w: i32, h: i32) -> bool {
        if w <= 0 || h <= 0 || self.data.is_empty() {
            return false;
        }
        let mut tdata = vec![0u8; (w * h * self.bytespp) as usize];
        let mut nscanline = 0;
        let mut oscanline = 0;
        let mut erry = 0;
        let nlinebytes = (w * self.bytespp) as usize;
        let olinebytes = (self.width * self.bytespp) as usize;

        for j in 0..self.height {
            let mut errx = self.width - w;
            let mut nx = -self.bytespp;
            let mut ox = -self.bytespp;
            for i in 0..self.width {
                ox += self.bytespp;
                errx += w;
                while errx >= self.width {
                    errx -= self.width;
                    nx += self.bytespp;
                    tdata[nscanline + nx as usize..nscanline + nx as usize + self.bytespp as usize]
                        .copy_from_slice(
                            &self.data[oscanline + ox as usize
                                ..oscanline + ox as usize + self.bytespp as usize],
                        );
                }
            }
            erry += h;
            oscanline += olinebytes;
            while erry >= self.height {
                if erry >= self.height * 2 {
                    tdata[nscanline + nlinebytes..nscanline + 2 * nlinebytes]
                        .copy_from_slice(&tdata[nscanline..nscanline + nlinebytes]);
                }
                erry -= self.height;
                nscanline += nlinebytes;
            }
        }
        self.data = tdata;
        self.width = w;
        self.height = h;
        true
    }

    pub fn get_bytespp(&self) -> i32 {
        self.bytespp
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn buffer(&self) -> &[u8] {
        &self.data
    }

    pub fn clear(&mut self) {
        self.data.fill(0);
    }
}

impl Drop for TGAImage {
    fn drop(&mut self) {
        // The `Vec<u8>` automatically manages memory, so no need for explicit deletion
    }
}

// Implement the TGAColor struct to match its use in TGAImage
#[derive(Debug, Default, Clone)]
pub struct TGAColor {
    raw: [u8; 4],
}

impl TGAColor {
    pub fn with_slice(p: &[u8], bpp: i32) -> Self {
        let mut color = TGAColor::default();
        color.raw[..bpp as usize].copy_from_slice(p);
        color
    }

    pub fn as_slice(&self, bpp: usize) -> &[u8] {
        &self.raw[..bpp]
    }

    pub fn as_mut_slice(&mut self, bpp: usize) -> &mut [u8] {
        &mut self.raw[..bpp]
    }
}

// Implement the TGAHeader struct to match its use in TGAImage
#[repr(C, packed)]
#[derive(Default, Debug)]
struct TGAHeader {
    idlength: u8,
    colormaptype: u8,
    datatypecode: u8,
    colormaporigin: u16,
    colormaplength: u16,
    colormapdepth: u8,
    x_origin: u16,
    y_origin: u16,
    width: u16,
    height: u16,
    bitsperpixel: u8,
    imagedescriptor: u8,
}

impl TGAHeader {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self as *mut _ as *mut u8, mem::size_of::<Self>()) }
    }

    fn as_ref(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self as *const _ as *const u8, mem::size_of::<Self>()) }
    }
}
