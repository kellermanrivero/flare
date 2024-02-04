use std::fmt;
use std::fs::File;
use std::os::fd::AsRawFd;

use memmap::{Mmap, MmapOptions};
use nix::ioctl_read_bad;

pub struct FrameBuffer {
    pub height: u32,
    pub width: u32,
    pub stride: u32,
    pub bits_per_pixel: u32,
    size: u32,
    file: File,
    map: Mmap,
}

impl FrameBuffer {
    pub fn get_contents(&self) -> &[u8] {
        self.map.as_ref()
    }
}

pub fn get_framebuffer() -> FrameBuffer {
    unsafe {
        let fb = File::open("/dev/fb0").expect("Cannot open /dev/fb0 framebuffer");
        let mut screeninfo: VarScreeninfo = Default::default();
        let fd = fb.as_raw_fd();

        // IOCTL
        fbioget_vscreeninfo(fd, &mut screeninfo).expect("IOCTL call failed!");

        // Get framebuffer size
        let size = screeninfo.yres * screeninfo.xres * (screeninfo.bits_per_pixel / 8);

        // Map the framebuffer to RAM
        let mapped_fb = MmapOptions::new().len(size as usize).map(&fb).expect("Cannot map the framebuffer");

        // Construct the framebuffer struct
        let framebuffer = FrameBuffer {
            height: screeninfo.yres,
            width: screeninfo.xres,
            stride: screeninfo.xres * (screeninfo.bits_per_pixel / 8),
            bits_per_pixel: screeninfo.bits_per_pixel,
            size,
            file: fb,
            map: mapped_fb,
        };

        // Box & Return
        framebuffer
    }
}

///Bitfield which is a part of VarScreeninfo.
#[repr(C)]
#[derive(Clone, Debug)]
struct Bitfield {
    pub offset: u32,
    pub length: u32,
    pub msb_right: u32,
}

///Struct as defined in /usr/include/linux/fb.h
#[repr(C)]
#[derive(Clone, Debug)]
struct VarScreeninfo {
    /* visible resolution		*/
    pub xres: u32,
    pub yres: u32,
    /* virtual resolution		*/
    pub xres_virtual: u32,
    pub yres_virtual: u32,
    /* offset from virtual to visible resolution */
    pub xoffset: u32,
    pub yoffset: u32,

    pub bits_per_pixel: u32,
    pub grayscale: u32,

    pub red: Bitfield,
    pub green: Bitfield,
    pub blue: Bitfield,
    pub transp: Bitfield,

    /* != 0 Non standard pixel format */
    pub nonstd: u32,

    /* see FB_ACTIVATE_*		*/
    pub activate: u32,

    /* height of picture in mm    */
    pub height: u32,
    /* width of picture in mm    */
    pub width: u32,

    /* (OBSOLETE) see fb_info.flags */
    pub accel_flags: u32,

    /* Timing: All values in pixclocks, except pixclock (of course) */

    /* pixel clock in ps (pico seconds) */
    pub pixclock: u32,

    pub left_margin: u32,
    pub right_margin: u32,
    pub upper_margin: u32,
    pub lower_margin: u32,
    pub hsync_len: u32,
    pub vsync_len: u32,
    pub sync: u32,
    pub vmode: u32,
    pub rotate: u32,
    pub colorspace: u32,
    pub reserved: [u32; 4],
}

impl Default for Bitfield {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl Default for VarScreeninfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl fmt::Display for FrameBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Width: {}\nHeight: {}\nStride: {}\nBits per pixel: {}\nSize in bytes: {}", self.width, self.height, self.stride, self.bits_per_pixel, self.size)
    }
}

const FBIOGET_VSCREENINFO: u16 = 0x4600;
ioctl_read_bad!(fbioget_vscreeninfo, FBIOGET_VSCREENINFO, VarScreeninfo);

