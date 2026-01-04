# FastSQL2Json

## 项目简介

**作者**: AlexBlair  
**作者网站**: [https://www.alexblair.org](https://www.alexblair.org)

A high-performance Rust-based middleware that solidifies MySQL query results (supporting time, JSON, binary, array formats and CTE complex logic) into JSON, and implements dual-write & caching mechanisms to achieve millisecond-level response for hot data interfaces under high concurrency.

一款基于 Rust 开发的高性能中间件，可将 MySQL 查询结果（支持时间、JSON、二进制、数组格式及 CTE 复杂查询逻辑）固化为 JSON 格式，并通过双写与缓存机制，实现高并发场景下热数据接口的毫秒级响应。

## 核心功能特性

- **高性能转换**：基于Rust语言实现，将MySQL查询结果高效转换为JSON格式
- **全面数据类型支持**：支持时间、JSON、二进制、数组格式以及CTE复杂逻辑
- **双写机制**：确保数据一致性和可靠性
- **智能缓存**：实现热数据缓存，大幅降低数据库压力
- **高并发处理**：采用异步设计，支持大规模并发请求
- **定时更新**：可配置不同SQL文件的更新间隔时间
- **原子写入**：确保JSON文件生成的完整性和一致性
- **灵活配置**：支持通过配置文件自定义各种参数

## 环境要求

- **Rust**: 1.70+ (2024 edition)
- **MySQL**: 5.7+ / MariaDB 10.2+
- **操作系统**: Linux / macOS / Windows

## 安装与编译步骤

### 1. 克隆仓库

```bash
git clone https://github.com/alexblair/FastSQL2Json.git
cd FastSQL2Json
```

### 2. 安装依赖

```bash
cargo build
```

### 3. 编译项目

```bash
# 开发构建
cargo build

# 生产构建（优化）
cargo build --release
```

### 4. 运行测试

```bash
cargo test
```

## 详细使用指南

### 配置说明

项目使用`config.toml`文件进行配置。首先复制示例配置文件并根据实际情况修改：

```bash
cp config.toml.example config.toml
```

#### 配置文件结构

```toml
[database]
host = "localhost"      # MySQL主机地址
port = 3306             # MySQL端口号
user = "root"            # MySQL用户名
password = "password"    # MySQL密码
database = "test_db"     # 数据库名称

[app]
start_dir = "./sql_files"  # SQL文件存放目录

[file_intervals]
# 设置特定SQL文件的生成间隔时间（分钟）
"./sql_files/query1.sql" = 60      # 每小时更新一次
"./sql_files/subdir/query2.sql" = 1440  # 每天更新一次
```

## 命令行选项

### 概述

FastSQL2Json提供了灵活的命令行选项，允许您根据需要调整应用程序的行为。以下是所有可用的命令行选项：

### 命令行语法

```bash
# 基本语法
FastSQL2Json [OPTIONS]

# 使用cargo运行
cargo run -- [OPTIONS]
```

### 可用选项

| 选项 | 长选项 | 描述 | 默认值 |
|------|--------|------|--------|
| `-c` | `--config` | 指定配置文件路径 | `config.toml`（当前目录） |
| `-q` | `--quiet` | 禁用所有日志输出 | 否 |
| `-e` | `--error-only` | 仅输出错误级别的日志 | 否 |
| `-h` | `--help` | 显示帮助信息 | - |
| `-V` | `--version` | 显示版本信息 | - |

### 详细说明

#### 1. 配置文件路径选项 `-c/--config`

**功能**：指定应用程序使用的配置文件路径。

**语法**：
```bash
FastSQL2Json -c <CONFIG_FILE_PATH>
FastSQL2Json --config <CONFIG_FILE_PATH>
```

**参数**：
- `<CONFIG_FILE_PATH>`：配置文件的绝对路径或相对路径（相对于当前工作目录）

**示例**：
```bash
# 使用相对路径指定配置文件
FastSQL2Json -c ./configs/my_config.toml

# 使用绝对路径指定配置文件
FastSQL2Json --config /etc/fastsql2json/config.toml

# 使用cargo运行时指定配置文件
cargo run -- -c ./test_config.toml
```

**说明**：
- 如果不指定此选项，应用程序将默认在当前目录下查找`config.toml`文件
- 配置文件必须遵循TOML格式，结构请参考[配置说明](#配置说明)部分

#### 2. 无输出选项 `-q/--quiet`

**功能**：禁用所有日志输出，使应用程序在静默模式下运行。

**语法**：
```bash
FastSQL2Json -q
FastSQL2Json --quiet
```

**示例**：
```bash
# 静默模式运行，不输出任何日志
FastSQL2Json -q

# 结合配置文件选项使用
FastSQL2Json -c ./config.toml -q
```

**说明**：
- 此选项适用于生产环境，减少日志输出带来的性能开销
- 即使启用静默模式，应用程序仍会正常执行所有功能，只是不输出日志信息
- 如果同时指定了`-q`和`-e`选项，`-q`将优先生效，禁用所有输出

#### 3. 仅输出错误选项 `-e/--error-only`

**功能**：仅输出错误级别的日志信息，忽略信息、警告等其他级别的日志。

**语法**：
```bash
FastSQL2Json -e
FastSQL2Json --error-only
```

**示例**：
```bash
# 仅输出错误日志
FastSQL2Json -e

# 结合配置文件选项使用
FastSQL2Json -c ./config.toml -e
```

**说明**：
- 此选项适用于生产环境，仅关注错误信息
- 错误日志包括数据库连接错误、SQL执行错误、文件写入错误等关键错误信息
- 正常情况下，应用程序会输出INFO级别的日志，如"Loaded configuration from config.toml"、"Connected to MySQL database"等

### 组合使用示例

您可以根据需要组合使用多个命令行选项：

```bash
# 使用指定配置文件，仅输出错误日志
FastSQL2Json -c ./prod_config.toml -e

# 使用指定配置文件，无任何输出
FastSQL2Json --config ./test_config.toml --quiet

# 查看帮助信息
FastSQL2Json -h

# 查看版本信息
FastSQL2Json -V
```

### 基本操作示例

#### 1. 准备SQL文件

在`sql_files`目录下创建SQL文件，例如`query1.sql`：

```sql
-- 查询用户信息
SELECT id, username, email, created_at
FROM users
WHERE status = 1
ORDER BY created_at DESC
LIMIT 10;
```

#### 2. 运行应用

```bash
# 默认方式运行（使用当前目录下的config.toml）
FastSQL2Json

# 使用开发构建运行
cargo run

# 使用生产构建运行
./target/release/FastSQL2Json

# 指定配置文件运行
./target/release/FastSQL2Json -c ./configs/prod.toml

# 静默模式运行生产构建
./target/release/FastSQL2Json -q
```

#### 3. 查看生成的JSON文件

应用运行后，会在与SQL文件相同的目录下生成对应的JSON文件，例如`query1.json`：

```json
[
  {
    "id": 1,
    "username": "user1",
    "email": "user1@example.com",
    "created_at": "2025-12-29T15:30:45Z"
  },
  {
    "id": 2,
    "username": "user2",
    "email": "user2@example.com",
    "created_at": "2025-12-28T14:20:30Z"
  }
]
```

### 命令行选项（中文说明）

#### 1. 配置文件路径选项 `-c/--config`

**功能**：指定应用程序使用的配置文件路径。

**语法**：
```bash
FastSQL2Json -c <配置文件路径>
FastSQL2Json --config <配置文件路径>
```

**参数**：
- `<配置文件路径>`：配置文件的绝对路径或相对路径（相对于当前工作目录）

**示例**：
```bash
# 使用相对路径指定配置文件
FastSQL2Json -c ./configs/my_config.toml

# 使用绝对路径指定配置文件
FastSQL2Json --config /etc/fastsql2json/config.toml
```

**说明**：
- 如果不指定此选项，应用程序将默认在当前目录下查找`config.toml`文件
- 配置文件必须遵循TOML格式，结构请参考[配置说明](#配置说明)部分

#### 2. 无输出选项 `-q/--quiet`

**功能**：禁用所有日志输出，使应用程序在静默模式下运行。

**语法**：
```bash
FastSQL2Json -q
FastSQL2Json --quiet
```

**示例**：
```bash
# 静默模式运行，不输出任何日志
FastSQL2Json -q

# 结合配置文件选项使用
FastSQL2Json -c ./config.toml -q
```

**说明**：
- 此选项适用于生产环境，减少日志输出带来的性能开销
- 即使启用静默模式，应用程序仍会正常执行所有功能，只是不输出日志信息
- 如果同时指定了`-q`和`-e`选项，`-q`将优先生效，禁用所有输出

#### 3. 仅输出错误选项 `-e/--error-only`

**功能**：仅输出错误级别的日志信息，忽略信息、警告等其他级别的日志。

**语法**：
```bash
FastSQL2Json -e
FastSQL2Json --error-only
```

**示例**：
```bash
# 仅输出错误日志
FastSQL2Json -e

# 结合配置文件选项使用
FastSQL2Json -c ./config.toml -e
```

**说明**：
- 此选项适用于生产环境，仅关注错误信息
- 错误日志包括数据库连接错误、SQL执行错误、文件写入错误等关键错误信息
- 正常情况下，应用程序会输出INFO级别的日志，如"Loaded configuration from config.toml"、"Connected to MySQL database"等

### 高级功能示例

#### 1. 支持CTE复杂逻辑

```sql
-- 使用CTE查询复杂数据
WITH recent_orders AS (
    SELECT user_id, SUM(amount) AS total_amount
    FROM orders
    WHERE created_at >= DATE_SUB(NOW(), INTERVAL 30 DAY)
    GROUP BY user_id
)
SELECT u.id, u.username, ro.total_amount
FROM users u
JOIN recent_orders ro ON u.id = ro.user_id
WHERE ro.total_amount > 1000
ORDER BY ro.total_amount DESC;
```

#### 2. 支持JSON数据类型

```sql
-- 查询包含JSON字段的数据
SELECT id, name, config->'$.theme' AS theme, config->'$.notifications' AS notifications
FROM user_settings
WHERE id = 1;
```

#### 3. 支持数组格式

```sql
-- 查询并生成数组格式结果
SELECT GROUP_CONCAT(tag_name) AS tags
FROM user_tags
WHERE user_id = 1;
```


## 常见问题解答

### Q: 应用运行时出现数据库连接错误怎么办？

A: 请检查`config.toml`文件中的数据库配置是否正确，包括主机地址、端口号、用户名、密码和数据库名称。同时确保MySQL服务正在运行，并且允许远程连接（如果需要）。

### Q: 如何调整并发处理的SQL文件数量？

A: 目前并发数固定为5，您可以修改`src/main.rs`文件中的`max_concurrent`变量来调整：

```rust
let max_concurrent = 5;  // 修改为您需要的并发数
```

### Q: 如何查看应用运行日志？

A: 应用使用`env_logger`库记录日志，您可以通过设置`RUST_LOG`环境变量来调整日志级别：

```bash
RUST_LOG=info cargo run  # 显示信息及以上级别的日志
RUST_LOG=debug cargo run # 显示调试及以上级别的日志
```

### Q: 生成的JSON文件权限问题如何解决？

A: 请确保应用程序对输出目录有写入权限。您可以使用`chmod`命令调整目录权限：

```bash
chmod 755 ./sql_files
```

## 贡献指南

我们欢迎社区贡献！如果您想参与项目开发，请遵循以下步骤：

1. Fork 仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

### 开发规范

- 遵循Rust官方代码风格
- 所有新功能必须包含测试
- 提交前运行`cargo fmt`和`cargo clippy`
- 编写清晰的提交信息

## 许可证信息

本项目采用 **CC BY-NC-SA 4.0**（署名-非商业性使用-相同方式共享 4.0 国际）许可证。

### 许可条款

- ✅ **允许**：二次修改、分发、展示、表演
- ❌ **禁止**：商业用途
- ✅ **要求**：署名、相同方式共享

### 详细说明

1. **署名**：必须给出适当的署名，提供指向许可证的链接，并表明是否对作品进行了修改。
2. **非商业性使用**：不得将本作品用于商业目的。
3. **相同方式共享**：如果您修改、转换本作品或者以本作品为基础进行创作，必须基于相同的许可证分发您的贡献作品。

完整的许可证文本可在[Creative Commons官网](https://creativecommons.org/licenses/by-nc-sa/4.0/deed.zh)查看。

## 联系方式
<div align="center">
  <a href="https://www.alexblair.org/">
    <img src="https://www.alexblair.org/404/AlexBlair_WX.png?From=GitHub_FastSQL2Json" alt="AlexBlair 图片" />
  </a>
</div>

- **项目主页**: [https://github.com/alexblair/FastSQL2Json](https://github.com/alexblair/FastSQL2Json)
- **Issues**: [https://github.com/alexblair/FastSQL2Json/issues](https://github.com/alexblair/FastSQL2Json/issues)


## 致谢

感谢所有为本项目做出贡献的开发者和社区成员！

---

**FastSQL2Json** - 高性能MySQL查询结果转JSON中间件
