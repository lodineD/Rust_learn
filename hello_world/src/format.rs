use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    // 返回"设置前景色为自身 RGB"的 ANSI 转义码，供 City 使用。
    fn fg_escape(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.red, self.green, self.blue)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // 用前景色（文字颜色）输出一个样例文本，展示这个颜色。
        write!(f, "\x1b[38;2;{};{};{}m████\x1b[0m",
               self.red, self.green, self.blue)
    }
}

struct City {
    name: &'static str,
    // Latitude
    lat: f32,
    // Longitude
    lon: f32,
    // color
    color: Color,
}

impl Display for City {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // 用城市颜色给文字上色：先输出颜色转义码，再输出文字，最后重置颜色。
        let c = self.color.fg_escape();
        write!(f, "{}{}: {:.3}°{} {:.3}°{}\x1b[0m",
               c, self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}



fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722, color: Color{red:128, green:255, blue:90}},
        City { name: "Oslo", lat: 59.95, lon: 10.75, color: Color{red:0, green:3, blue:254}},
        City { name: "Vancouver", lat: 49.25, lon: -123.1, color: Color{red:0, green:0, blue:0}},
    ] {
        println!("{}", city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 128, green: 255, blue: 90 },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", color);
    }
}