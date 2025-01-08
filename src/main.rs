use num::bigint::{BigInt, Sign::Plus};

#[derive(Debug, Default, Clone)]
struct Crosshair {
    gap: i8,
    outline_enabled: bool,
    r: u8,
    g: u8,
    b: u8,
    alpha_enabled: bool,
    alpha: u8,
    split_distance: u8,
    fixed_gap: i8,
    color: u8,
    outline: f32,
    inner_alpha: f32,
    outer_alpha: f32,
    split_size: f32,
    thickness: f32,
    dot: bool,
    weapon_gap: bool,
    t_style: bool,
    style: u8,
    size: f32,
}

fn main() {
    let mut crosshair = Crosshair {
        ..Default::default()
    };
    const DICTIONARY: &str = "ABCDEFGHJKLMNOPQRSTUVWXYZabcdefhijkmnopqrstuvwxyz23456789";
    let crosshair_code = "CSGO-yxFVd-8AmDF-xw85f-8t55O-ey3JE";
    let crosshair_data = crosshair_code
        .replace("CSGO-", "")
        .replace('-', "")
        .chars()
        .rev()
        .collect::<Vec<_>>();

    let mut big = BigInt::new(Plus, Vec::new());

    for (_i, &c) in crosshair_data.iter().enumerate() {
        if let Some(index) = DICTIONARY.chars().position(|x| x == c) {
            big = big * (DICTIONARY.len() as u32) as i32 + index as i32;
        }
    }

    let crosshair_data = big.to_bytes_be().1;

    crosshair.gap = crosshair_data[2] as i8 / 10;
    crosshair.outline_enabled = crosshair_data[10] & 8 == 8;
    crosshair.thickness = crosshair_data[12] as f32 / 10.0;
    crosshair.size = crosshair_data[14] as f32 / 10.0;
    crosshair.r = crosshair_data[4];
    crosshair.g = crosshair_data[5];
    crosshair.b = crosshair_data[6];
    crosshair.alpha = crosshair_data[7];
    crosshair.dot = (crosshair_data[13] >> 4) & 1 == 1;
    crosshair.color = crosshair_data[10] & 7;
    crosshair.weapon_gap = (crosshair_data[13] >> 4) & 2 == 2;
    crosshair.alpha_enabled = (crosshair_data[13] >> 4) & 4 == 4;
    crosshair.t_style = (crosshair_data[13] >> 4) & 8 == 8;
    crosshair.style = crosshair_data[13] & 0xf >> 1;
    crosshair.fixed_gap = crosshair_data[9] as i8 / 10;
    crosshair.split_distance = crosshair_data[8];
    crosshair.outline = crosshair_data[3] as f32 / 2.0;
    crosshair.inner_alpha = (crosshair_data[10] >> 4) as f32 / 10.0;
    crosshair.outer_alpha = (crosshair_data[11] & 0xf) as f32 / 10.0;
    crosshair.split_size = (crosshair_data[11] >> 4) as f32 / 10.0;

    get_commands(&crosshair);
}

fn get_commands(crosshair: &Crosshair) {
    println!("cl_crosshair_drawoutline {}
cl_crosshair_dynamic_maxdist_splitratio {}
cl_crosshair_dynamic_splitalpha_innermod {}
cl_crosshair_dynamic_splitalpha_outermod {}
cl_crosshair_dynamic_splitdist {}
cl_crosshair_outlinethickness {}
cl_crosshair_t {}
cl_crosshairalpha {}
cl_crosshaircolor {}
cl_crosshaircolor_b {}
cl_crosshaircolor_g {}
cl_crosshaircolor_r {}
cl_crosshairdot {}
cl_crosshairgap {}
cl_crosshairgap_useweaponvalue {}
cl_crosshairsize {}
cl_crosshairstyle {}
cl_crosshairthickness {}
cl_crosshairusealpha {}
cl_fixedcrosshairgap {}",
        crosshair.outline_enabled,
        crosshair.split_size,
        crosshair.inner_alpha,
        crosshair.outer_alpha,
        crosshair.split_distance,
        crosshair.outline,
        crosshair.t_style,
        crosshair.alpha,
        crosshair.color,
        crosshair.b,
        crosshair.g,
        crosshair.r,
        crosshair.dot,
        crosshair.gap,
        crosshair.weapon_gap,
        crosshair.size,
        crosshair.style,
        crosshair.thickness,
        crosshair.alpha_enabled,
        crosshair.fixed_gap
    );
}
