use cobble::{buffer::Buffer, vector2::Vec2i};
use cobble::cobble::*;

mod cobble;

fn main() { 
    let mut buffer: Buffer = Buffer::new(64, 32);
    
    cbl_draw_tri(&mut buffer, ' ', Vec2i{x: 0, y: 0}, Vec2i{x: 0, y: 12}, Vec2i{x: 12, y: 12});

    //cbl_draw_rect(&mut buffer, ' ', Vec2i{x: 1, y: 1}, 10, 20);
    
    //cbl_draw_line(&mut buffer, ' ', Vec2i{x: 4, y: 5}, Vec2i{x: 10, y: 11});

    cbl_display(&buffer);
    
}
