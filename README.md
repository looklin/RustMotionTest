# RustMotionTest

通用运动控制板卡抽象层 — Rust 实现。

通过 Trait 抽象兼容主流运动控制板卡（雷赛、固高、研华），提供统一的运动控制接口。

## 特性

- **统一接口** — `MotionCard` + `Axis` Trait 屏蔽厂商差异
- **多厂商支持** — 雷赛 LTSMC / 固高 GTS / 研华 PCI-124x
- **Mock 适配器** — 无硬件时可开发、测试、调试
- **工厂模式** — 运行时动态创建板卡实例
- **FFI 绑定** — 通过 `extern "C"` 直接调用厂商 SDK
- **零额外依赖** — 核心库仅依赖 `thiserror`
- **跨平台** — 支持 Windows (`.dll`) 和 Linux (`.so`)

## 项目结构

```
RustMotionTest/
├── motion-core/          # 核心库 — Trait、模型、错误、工厂
├── motion-mock/          # 模拟适配器（无硬件开发/测试）
├── motion-leisai/        # 雷赛 LTSMC 适配器
├── motion-googol/        # 固高 GTS 适配器
├── motion-advantech/     # 研华 PCI-124x 适配器
└── motion-cli/           # CLI 演示程序
```

## 快速开始

### 依赖

- Rust 1.70+ (MSRV)
- 各厂商 SDK 库（`.dll` / `.so`，仅实际使用时需要）

### 构建

```bash
cargo build
```

### 运行 CLI 演示（使用 Mock）

```bash
cargo run -p motion-cli
```

### 测试

```bash
# 测试纯 Rust 组件（无需硬件 SDK）
cargo test -p motion-core -p motion-mock
```

## 使用示例

```rust
use motion_core::*;

// 1. 创建工厂并注册适配器
let mut factory = CardFactory::new();
factory.register("Mock", Box::new(|| {
    Box::new(motion_mock::MockCard::new())
}));

// 2. 创建板卡实例
let mut card = factory.create(CardVendor::Mock)?;

// 3. 初始化
let config = CardConfig::default()
    .with_axis_count(4)
    .with_timeout(5000);
card.initialize(&config)?;

// 4. 轴使能
card.get_axis(0)?.enable();

// 5. 运动控制
card.move_absolute(0, 100.0, 50.0)?;    // 绝对定位
card.move_relative(0, 50.0, 30.0)?;     // 相对移动
card.get_axis(0)?.jog_positive(80.0);   // 点动
card.stop_axis(0, StopMode::Immediate);  // 停止

// 6. IO 操作
card.write_output(0, true);              // 输出
let input = card.read_input(0);          // 输入

// 7. 回零
card.move_home(0, HomeMode::Auto)?;

// 8. 关闭
card.close();
```

## 支持板卡

| 厂商 | 系列 | Crate | 状态 |
|------|------|-------|------|
| 雷赛 Leadshine | LTSMC | `motion-leisai` | ✅ FFI 完成 |
| 固高 Googol | GTS | `motion-googol` | ✅ FFI 完成 |
| 研华 Advantech | PCI-124x | `motion-advantech` | ✅ FFI 完成 |
| Mock（模拟） | — | `motion-mock` | ✅ 完整实现 |

## 核心 Trait

### MotionCard — 板卡统一接口

```rust
pub trait MotionCard: Send + Sync {
    fn initialize(&mut self, config: &CardConfig) -> Result<()>;
    fn close(&mut self);
    fn status(&self) -> CardStatus;
    fn card_name(&self) -> &str;
    fn axis_count(&self) -> usize;
    fn get_axis(&mut self, index: usize) -> Result<&mut dyn Axis>;
    fn read_input(&self, port: u16) -> bool;
    fn write_output(&mut self, port: u16, value: bool);
    fn move_absolute(&mut self, axis: usize, pos: f64, vel: f64) -> Result<()>;
    fn move_relative(&mut self, axis: usize, dist: f64, vel: f64) -> Result<()>;
    fn move_home(&mut self, axis: usize, mode: HomeMode) -> Result<()>;
    fn stop_axis(&mut self, axis: usize, mode: StopMode);
    fn stop_all(&mut self);
    // ...
}
```

### Axis — 轴统一接口

```rust
pub trait Axis: Send + Sync {
    fn axis_index(&self) -> usize;
    fn is_enabled(&self) -> bool;
    fn is_moving(&self) -> bool;
    fn current_position(&self) -> f64;
    fn current_velocity(&self) -> f64;
    fn enable(&mut self);
    fn disable(&mut self);
    fn move_absolute(&mut self, pos: f64, vel: f64);
    fn move_relative(&mut self, dist: f64, vel: f64);
    fn jog_positive(&mut self, vel: f64);
    fn jog_negative(&mut self, vel: f64);
    fn stop(&mut self, mode: StopMode);
    fn move_home(&mut self, mode: HomeMode);
    // ...
}
```

## 添加新板卡

1. 创建新 crate（如 `motion-xxx`）
2. 在 `src/bindings.rs` 中声明 FFI 函数
3. 实现 `MotionCard` 和 `Axis` Trait
4. 在 `Cargo.toml` 中加入 workspace members
5. 注册到 `CardFactory`

## 许可证

MIT
