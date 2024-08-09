pub struct Buffer {
    pub data: Vec<Vec<char>>,
    pub width: u32,
    pub height: u32  
}

impl Buffer {
    pub fn new(w: u32, h: u32) -> Buffer {
        let mut  _data: Vec::<Vec<char>> = Vec::new();

        for _x in 0..w {
            let mut col: Vec<char> = Vec::new();

            for _y in 0..h {
                col.push('█');
            }

            _data.push(col);
        }

        return Buffer {
            data: _data,
            width: w,
            height: h
        };
    }
}
