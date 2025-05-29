// 新的统一接口
pub trait Shape {
    fn draw(&self);
}

// 旧系统的具体类，接口不兼容 ----------------------------
pub struct LegacyRectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl LegacyRectangle {
    fn draw_old(&self) {
        println!(
            "LegacyRectangle: Drawing from ({}, {}) to ({}, {})",
            self.x,
            self.y,
            self.x + self.width,
            self.y + self.height
        );
    }
}

// 适配器：将 LegacyRectangle 适配为 Shape 接口 ------------------------
pub struct RectangleAdapter {
    pub legacy_rect: LegacyRectangle,
}

impl Shape for RectangleAdapter {
    fn draw(&self) {
        self.legacy_rect.draw_old();
    }
}
//-------------------------------------------------------------------
// 使用示例
fn main() {
    let legacy_rect = LegacyRectangle {
        x: 10,
        y: 20,
        width: 100,
        height: 50,
    };

    let adapter = RectangleAdapter {
        legacy_rect: legacy_rect,
    };

    let shapes: Vec<&dyn Shape> = vec![&adapter];

    for shape in shapes {
        shape.draw();
    }
}