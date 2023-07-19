## Embedded TSDB High-Level Design

#### Goals

1. 提供一个嵌入式的时间序列存储数据库，用于边缘计算场景下设备上报数据的存储与计算
2. 提供数据库模型，模型支持一下特性
    + 提供多租户，每个 Database 属于一个租户
    + 提供一个大表，每张大表用来定义一个设备模型[1]，大表的 scheme 包括设备 tags 和 fields
    + 大表包含小表，每张小表表示一个设备实例，每个小表拥有具体 tags 且 tags 不同
    + 提供常见数据类型以及 geo 相关类型与函数
3. 提供基于大表与小表的查询
4. 融合流式数据库部分能力，提供数据订阅功能

#### Non-Goals

1. 分布式
2. 提供数据流式处理函数

#### Design choices

1. 分离元数据存储层(meta)与数据存储层(storage)，以便过后分布式版本
2. 实现 sql 标准，并提供执行 sql 的入口，参考 sqlite
3. 对外暴露良好的元数据层(meta) API，用于与边缘计算平台结合
4. 使用 rust 语言实现，推荐使用库 arrow-rs, datafusion, sled, geoarrow 实现

#### Appendix

[1] 设备模型
```json
{
    "id": "1000",
    "name": "温度计模板",
    "manufacturer": "制造商",
    "namespace": "namespace",
    "fields": [
      {
        "id": 21,
        "name": "温度",
        "description": "温度计显示温度",
        "resource_scheme": "Telemetry",
        "property": {
          "value": {
            "value_type": "INT",
            "access_mode": "ReadOnly",
            "default_value": "26",
            "base": "1",
            "scale": 1.0,
            "offset": 0
          },
          "units": {
            "name": "摄氏度"
          }
        }
      }
    ],
    "tags": {
      "a": "string",
      "b": "int"
    }
}
```

