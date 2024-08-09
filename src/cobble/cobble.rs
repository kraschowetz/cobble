use std::vec::Vec;
use crate::cobble::buffer::*;
use crate::cobble::vector2::*;

#[allow(dead_code)]
pub fn cbl_display(buffer: &Buffer) {
    for y in 0..buffer.height {
        for x in 0..buffer.width {
            print!("{}", buffer.data[x as usize][y as usize]);
        }
        print!("\n");
    }    
}

#[allow(dead_code)]
pub fn cbl_set_px(buffer: &mut Buffer, px: char, pos: Vec2i) {
    if pos.x as u32 >= buffer.width || pos.y as u32 >= buffer.height {
        return
    }

    buffer.data[pos.x as usize][pos.y as usize] = px;
}

#[allow(dead_code)]
pub fn cbl_draw_line(buffer: &mut Buffer, px: char, p1: Vec2i, p2: Vec2i) {
    let len: i32 = (p2 - p1).as_vec2f().len() as i32 + 1;
    let dir: Vec2f = (p2 - p1).as_vec2f().normalized();

    for i in 0..len {
        cbl_set_px(buffer, px, p1 + (dir * i as f32).as_vec2i())
    }
}

#[allow(dead_code)]
pub fn cbl_draw_tri(buffer: &mut Buffer, px: char, p1: Vec2i, p2: Vec2i, p3: Vec2i) {
    cbl_draw_line(buffer, px, p1, p2);
    cbl_draw_line(buffer, px, p1, p3);
    cbl_draw_line(buffer, px, p3, p2);
}

#[allow(dead_code)]
pub fn cbl_draw_rect(buffer: &mut Buffer, px: char, pos: Vec2i, width: i32, height: i32) {
    cbl_draw_line(buffer, px, pos, Vec2i{x: pos.x + width, y: pos.y});
    cbl_draw_line(buffer, px, pos, Vec2i{x: pos.x, y: pos.y + height});
    cbl_draw_line(buffer, px, Vec2i{x: pos.x, y: pos.y + height}, Vec2i{x: pos.x + width, y: pos.y + height});
    cbl_draw_line(buffer, px, Vec2i{x: pos.x + width, y: pos.y}, Vec2i{x: pos.x + width, y: pos.y + height});
}

#[allow(dead_code)]
pub fn cbl_draw_quad(buffer: &mut Buffer, px: char, p1: Vec2i, p2: Vec2i, p3: Vec2i, p4: Vec2i) {
    cbl_draw_line(buffer, px, p1, p2);
    cbl_draw_line(buffer, px, p1, p3);
    cbl_draw_line(buffer, px, p3, p4);
    cbl_draw_line(buffer, px, p2, p4)
}

#[allow(dead_code)]
pub fn cbl_draw_polygon(buffer: &mut Buffer, px: char, points: Vec<Vec2i>) {
    let len = points.len();

    for i in 1..len {
        cbl_draw_line(buffer, px, points[i - 1], points[i]);
    }
}
