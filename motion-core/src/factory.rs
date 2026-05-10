//! 板卡工厂 — 根据厂商创建对应适配器实例

use crate::error::{MotionError, Result};
use crate::models::CardVendor;
use crate::traits::MotionCard;

/// 板卡工厂函数类型
type CardFactoryFn = Box<dyn Fn() -> Box<dyn MotionCard> + Send + Sync>;

/// 板卡工厂
pub struct CardFactory {
    registry: std::collections::HashMap<String, CardFactoryFn>,
}

impl CardFactory {
    /// 创建空工厂 (适配器由使用方注册)
    pub fn new() -> Self {
        Self {
            registry: std::collections::HashMap::new(),
        }
    }

    /// 注册自定义板卡工厂
    pub fn register(&mut self, name: &str, factory_fn: CardFactoryFn) {
        self.registry.insert(name.to_string(), factory_fn);
    }

    /// 根据厂商创建板卡实例
    pub fn create(&self, vendor: CardVendor) -> Result<Box<dyn MotionCard>> {
        let key = match vendor {
            CardVendor::Leisai => "Leisai",
            CardVendor::Googol => "Googol",
            CardVendor::Advantech => "Advantech",
            CardVendor::Mock => "Mock",
        };

        let factory_fn = self.registry.get(key).ok_or_else(|| {
            MotionError::UnsupportedVendor(format!(
                "未注册板卡 '{}', 请检查依赖或手动注册",
                key
            ))
        })?;

        Ok(factory_fn())
    }

    /// 根据名称创建板卡实例
    pub fn create_by_name(&self, name: &str) -> Result<Box<dyn MotionCard>> {
        let factory_fn = self.registry.get(name).ok_or_else(|| {
            MotionError::UnsupportedVendor(format!("未知板卡: {}", name))
        })?;

        Ok(factory_fn())
    }

    /// 列出所有已注册的板卡
    pub fn list_vendors(&self) -> Vec<&str> {
        self.registry.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for CardFactory {
    fn default() -> Self {
        Self::new()
    }
}
