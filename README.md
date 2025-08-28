# ZRDDS SDK

ZRDDS 的 Rust SDK，为高性能 DDS 通信提供安全的 Rust 绑定。

## 安装

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
zrdds = "0.1.0"
```

## 运行示例

```bash
# 编译示例
cargo build --example basic_usage

# 运行示例
cargo run --example basic_usage
```

## 快速开始

```rust
use zrdds;
use zrdds::bindings::*;
use std::ffi::CString;

fn main() {
    // 使用配置初始化 ZRDDS
    let factory = zrdds::init("ZRDDS_QOS_PROFILES.xml", "non_rio");
    
    // 创建域参与者
    let dp = factory.create_domain_participant(150, "udp_dp");
    
    // 创建发布者/写者
    let writer = dp.publish(
        "MyTopic",
        unsafe { &raw mut DDS_BytesTypeSupport_instance },
        "non_zerocopy_reliable",
    );
    
    // 使用 bytes_write 工具函数写入数据
    let data = b"Hello DDS";
    let ret = zrdds::bytes_write(150, "MyTopic", data);
    if ret != 0 {
        panic!("写入数据失败");
    }
}
```

## API 文档

### 核心函数

#### `zrdds::init(xml: &str, profile: &str) -> DomainParticipantFactory`

初始化 ZRDDS 系统，返回域参与者工厂实例。

**参数:**
- `xml`: XML 配置文件路径
- `profile`: 要使用的 QoS 配置文件名称

**返回:** `DomainParticipantFactory` 实例，用于创建域参与者

#### `zrdds::bytes_write(domain_id: u32, topic_name: &str, data: &[u8]) -> u32`

向指定主题写入字节数据的便捷函数。

**参数:**
- `domain_id`: 域 ID
- `topic_name`: 主题名称
- `data`: 要写入的字节数据

**返回:** 操作结果码，0 表示成功

### 核心类型

#### `DomainParticipantFactory`

域参与者工厂，负责创建和管理域参与者。

**方法:**
- `create_domain_participant(domain_id: u32, participant_name: &str) -> DomainParticipant`
  - 创建域参与者实例
  - `domain_id`: 域 ID
  - `participant_name`: 参与者名称

#### `DomainParticipant`

域参与者，DDS 通信的核心实体。

**方法:**
- `publish(topic_name: &str, type_support: *mut DDS_TypeSupport, qos_name: &str) -> DataWriter`
  - 创建数据写者用于发布数据
  - `topic_name`: 主题名称
  - `type_support`: DDS 类型支持指针
  - `qos_name`: QoS 配置文件名称

#### `DataWriter`

数据写者，用于向主题发布数据。

#### `DataReader`

数据读者，用于从主题订阅数据。

## 示例

运行基本使用示例:

```bash
cargo run --example basic_usage
```

## 构建

项目要求:
- Rust 2024 edition 或更高版本
- `libs` 目录中的 ZRDDS C 库文件
- `include` 目录中的头文件

构建项目:

```bash
cargo build
```

运行测试:

```bash
cargo test
```

## 架构设计

本 SDK 采用分层设计:

1. **底层绑定层** (`bindings`): 自动生成的 C FFI 绑定
2. **核心抽象层** (`zrdds`): 提供安全的 Rust API 封装
   - `DomainParticipantFactory`: 工厂模式管理域参与者
   - `DomainParticipant`: 域参与者抽象
   - `DataWriter`/`DataReader`: 数据读写抽象

## 安全性

本 SDK 在不安全的 C FFI 调用基础上提供安全的 Rust 封装。但某些操作仍需要谨慎处理原始指针和内存管理。请确保:

- 正确初始化和清理 DDS 资源
- 避免在多线程环境中不当共享原始指针
- 遵循 DDS 的生命周期管理规则

## 许可证

本项目采用 MIT 许可证。详见 LICENSE 文件。