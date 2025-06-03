/// 
/// Bridge 解耦抽象和其实现 可用 trait + 组合结构体实现
/// 优势             说明
/// 解耦抽象与实现	   图形和渲染器可独立扩展
/// 避免类爆炸	      不需要为每个图形和渲染器的组合创建子类
/// 支持运行时切换	   可动态更换渲染器或图形

// ================== 实现部分接口（Implementor）==================
/// 渲染接口
pub trait Renderer {
    fn render_circle(&self, radius: f64);
}

// ================== 具体实现类（Concrete Implementors）==================
/// OpenGL 渲染器
pub struct OpenGLRenderer;

impl Renderer for OpenGLRenderer {
    fn render_circle(&self, radius: f64) {
        println!("OpenGL: Rendering circle with radius {}", radius);
    }
}

/// DirectX 渲染器
pub struct DirectXRenderer;

impl Renderer for DirectXRenderer {
    fn render_circle(&self, radius: f64) {
        println!("DirectX: Rendering circle with radius {}", radius);
    }
}

// ================== 抽象类（Abstraction）==================
/// 圆形类
pub struct Circle {
    /**
     *  渲染器接口，可以接收实现子类
     *  Send	类型的所有权可以在线程之间安全转移
     *  Sync	类型的引用（&T）可以被多个线程同时访问
     */
    renderer: Box<dyn Renderer + Send + Sync>, // 桥接到实现
    radius: f64,
}

impl Circle {
    ///  创建圆形
    /**
     * renderer: 渲染器接口, 可以接收实现子类
     * radius: 半径, 默认为1.0
     */ 
    pub fn new(renderer: Box<dyn Renderer + Send + Sync>, radius: f64) -> Self {
        Self { renderer, radius }
    }
    /// 绘制圆形
    pub fn draw(&self) {
        self.renderer.render_circle(self.radius);
    }
}

fn main() {
    // 创建OpenGL渲染器、创建DirectX渲染器
    let opengl_renderer = Box::new(OpenGLRenderer);
    let directx_renderer : Box<dyn Renderer + Send + Sync> = Box::new(DirectXRenderer);

    // 创建圆形
    let circle1 = Circle::new(opengl_renderer, 5.0);
    let circle2 = Circle::new(directx_renderer, 10.0);
    //  绘制圆形
    circle1.draw();
    circle2.draw();
}