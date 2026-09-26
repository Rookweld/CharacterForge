use std::{env, fs, path::PathBuf};

const ICON_SIZE: usize = 32;
const PIXEL_BYTES: usize = ICON_SIZE * ICON_SIZE * 4;
const MASK_BYTES: usize = ICON_SIZE * 4 * ICON_SIZE;

fn push_u16(bytes: &mut Vec<u8>, value: u16) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn set_pixel(pixels: &mut [[u8; 4]], x: usize, y: usize, color: [u8; 4]) {
    pixels[y * ICON_SIZE + x] = color;
}

fn make_window_icon() -> Vec<u8> {
    let mut pixels = vec![[0, 0, 0, 0]; ICON_SIZE * ICON_SIZE];
    let graphite = [25, 27, 29, 255];
    let ember = [184, 134, 93, 255];

    for y in 0..ICON_SIZE {
        for x in 0..ICON_SIZE {
            if (5..27).contains(&x) && (5..27).contains(&y) {
                set_pixel(&mut pixels, x, y, graphite);
            }
        }
    }

    let letters = [
        [
            0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110,
        ],
        [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
    ];
    for (character, rows) in letters.iter().enumerate() {
        for (row, bits) in rows.iter().enumerate() {
            for column in 0..5 {
                if bits & (1 << (4 - column)) != 0 {
                    for dy in 0..2 {
                        for dx in 0..2 {
                            set_pixel(
                                &mut pixels,
                                8 + character * 12 + column * 2 + dx,
                                9 + row * 2 + dy,
                                ember,
                            );
                        }
                    }
                }
            }
        }
    }

    let mut icon = Vec::with_capacity(22 + 40 + PIXEL_BYTES + MASK_BYTES);
    push_u16(&mut icon, 0);
    push_u16(&mut icon, 1);
    push_u16(&mut icon, 1);
    icon.extend_from_slice(&[ICON_SIZE as u8, ICON_SIZE as u8, 0, 0]);
    push_u16(&mut icon, 1);
    push_u16(&mut icon, 32);
    push_u32(&mut icon, (40 + PIXEL_BYTES + MASK_BYTES) as u32);
    push_u32(&mut icon, 22);

    push_u32(&mut icon, 40);
    push_u32(&mut icon, ICON_SIZE as u32);
    push_u32(&mut icon, (ICON_SIZE * 2) as u32);
    push_u16(&mut icon, 1);
    push_u16(&mut icon, 32);
    push_u32(&mut icon, 0);
    push_u32(&mut icon, (PIXEL_BYTES + MASK_BYTES) as u32);
    push_u32(&mut icon, 0);
    push_u32(&mut icon, 0);
    push_u32(&mut icon, 0);
    push_u32(&mut icon, 0);

    for row in pixels.chunks_exact(ICON_SIZE).rev() {
        for [red, green, blue, alpha] in row {
            icon.extend_from_slice(&[*blue, *green, *red, *alpha]);
        }
    }
    icon.resize(icon.len() + MASK_BYTES, 0);
    icon
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let icon_path =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set"))
            .join("icons")
            .join("icon.ico");
    fs::create_dir_all(icon_path.parent().expect("icon has a parent directory"))
        .expect("create generated icon directory");
    let icon_bytes = make_window_icon();
    if fs::read(&icon_path).ok().as_deref() != Some(icon_bytes.as_slice()) {
        fs::write(&icon_path, icon_bytes).expect("write generated Windows window icon");
    }
    println!("cargo:rerun-if-changed=icons/icon.ico");

    let windows = tauri_build::WindowsAttributes::new().window_icon_path(icon_path);
    let attributes = tauri_build::Attributes::new().windows_attributes(windows);
    tauri_build::try_build(attributes).expect("failed to configure the Tauri application");
}
