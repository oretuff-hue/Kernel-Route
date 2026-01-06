#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[repr(C)]
struct MultibootTag {
    typ: u32,
    size: u32,
}

const TAG_FRAMEBUFFER: u32 = 8;

#[repr(C)]
struct FramebufferTag {
    typ: u32,
    size: u32,
    addr: u64,
    pitch: u32,
    width: u32,
    height: u32,
    bpp: u8,
    fb_type: u8,
    _reserved: u16,
}

fn find_framebuffer(mb_addr: usize) -> &'static FramebufferTag {
    let mut tag = (mb_addr + 8) as *const MultibootTag;

    loop {
        unsafe {
            if (*tag).typ == TAG_FRAMEBUFFER {
                return &*(tag as *const FramebufferTag);
            }
            if (*tag).typ == 0 {
                break;
            }
            tag = ((tag as usize + (*tag).size as usize + 7) & !7) as *const MultibootTag;
        }
    }

    panic!("no framebuffer");
}

fn put_pixel(fb: &FramebufferTag, x: u32, y: u32, color: u32) {
    let ptr = fb.addr as *mut u8;
    let offset = (y * fb.pitch + x * 4) as isize;
    unsafe {
        *(ptr.offset(offset) as *mut u32) = color;
    }
}

#[no_mangle]
pub extern "C" fn kernel_main(mb_addr: usize) -> ! {
    let fb = find_framebuffer(mb_addr);

    // limpa tela (preto)
    for y in 0..fb.height {
        for x in 0..fb.width {
            put_pixel(fb, x, y, 0x00000000);
        }
    }

    // retângulo branco (teste)
    for y in 200..400 {
        for x in 300..700 {
            put_pixel(fb, x, y, 0x00FFFFFF);
        }
    }

    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}
  