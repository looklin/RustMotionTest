//! 运动控制板卡 CLI 演示程序

use motion_core::*;

fn main() {
    println!("🏭 RustMotionTest CLI");
    println!("======================\n");

    // 创建工厂并注册 Mock 适配器
    let mut factory = CardFactory::new();
    factory.register("Mock", Box::new(|| {
        Box::new(motion_mock::MockCard::new())
    }));

    // === 初始化演示 ===
    println!("1️⃣ 初始化模拟板卡...");
    let mut card = factory.create(CardVendor::Mock).unwrap();
    let config = CardConfig::new().with_axis_count(4);
    card.initialize(&config).unwrap();

    println!("   名称: {}", card.card_name());
    println!("   类型: {}", card.card_type());
    println!("   轴数: {}", card.axis_count());

    // === 轴使能 ===
    println!("\n2️⃣ 轴使能...");
    card.get_axis(0).unwrap().enable();
    println!("   轴 0 已使能 ✓");
    println!("   状态: enabled={}, moving={}",
        card.get_axis(0).unwrap().is_enabled(),
        card.get_axis(0).unwrap().is_moving());

    // === 运动演示 ===
    println!("\n3️⃣ 运动控制...");
    card.move_absolute(0, 100.0, 50.0).unwrap();
    println!("   绝对定位 → 位置: 100.0");
    println!("   当前位置: {}", card.get_axis(0).unwrap().current_position());

    card.move_relative(0, 50.0, 30.0).unwrap();
    println!("   相对移动 +50.0");
    println!("   当前位置: {}", card.get_axis(0).unwrap().current_position());

    // === 点动 ===
    println!("\n4️⃣ 点动测试...");
    card.get_axis(0).unwrap().jog_positive(80.0);
    println!("   正向点动中...");
    card.stop_axis(0, StopMode::Immediate);
    println!("   已停止 ✓");

    // === IO 测试 ===
    println!("\n5️⃣ IO 测试...");
    card.write_output(0, true);
    println!("   输出端口 0 → true");
    println!("   输入端口 0 → {}", card.read_input(0));

    // === 回零 ===
    println!("\n6️⃣ 回零...");
    card.move_home(0, HomeMode::Auto).unwrap();
    println!("   当前位置: {}", card.get_axis(0).unwrap().current_position());

    // === 状态 ===
    println!("\n7️⃣ 板卡状态...");
    let status = card.status();
    println!("   已初始化: {}", status.is_initialized);
    println!("   运行时间: {:.1}ms", status.uptime.as_secs_f64() * 1000.0);

    card.close();
    println!("\n✅ 全部演示完成!");
}
