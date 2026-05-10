# RustMotionTest — 通用运动控制板卡抽象层 (Rust 实现)

## 产品概述

与 MotionTest (C#) 相同功能，使用 Rust 实现。通过 Trait 抽象兼容主流运动控制板卡 SDK。

## 架构

```
业务代码 → MotionCard Trait → 适配器实现 → 厂商 SDK (FFI)
```

## 项目结构

```
RustMotionTest/
├── Cargo.toml                      # Workspace
│
├── motion-core/                    # 核心库
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                  # 导出
│       ├── traits.rs               # MotionCard / Axis Trait
│       ├── models.rs               # 数据模型
│       ├── error.rs                # 错误类型
│       ├── config.rs               # 配置
│       └── factory.rs              # 工厂
│
├── motion-mock/                    # 模拟适配器
│   ├── Cargo.toml
│   └── src/lib.rs
│
├── motion-leisai/                  # 雷赛适配器 (LTSMC.dll FFI)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── bindings.rs             # FFI 声明
│
├── motion-googol/                  # 固高适配器 (gts.dll FFI)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── bindings.rs
│
├── motion-advantech/               # 研华适配器 (pci124x.dll FFI)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── bindings.rs
│
├── motion-cli/                     # CLI 演示程序
│   ├── Cargo.toml
│   └── src/main.rs
│
└── README.md
```

## 核心 Trait

```rust
/// 板卡统一接口
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
}

/// 轴统一接口
pub trait Axis {
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
}
```

## FFI 绑定

使用 `extern "C"` 声明各厂商 SDK 函数：

```rust
// 雷赛 LTSMC.dll
extern "C" {
    fn nmc_board_init() -> i16;
    fn nmc_set_axis_enable(card: u16, axis: u16, enable: bool) -> i16;
    fn nmc_axis_move_absolute(card: u16, axis: u16, pos: f64, vel: f64) -> i16;
    // ...
}
```

## 渐进特性

| Rust 版本 | 特性 |
|-----------|------|
| 1.70+ (MSRV) | 基础功能 |
| 1.75+ | let chains |
| 1.80+ | trait upcasting |

## 跨平台

- Windows: 加载 `.dll`
- Linux: 加载 `.so`
- 通过 `libloading` crate 动态加载
