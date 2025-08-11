# Qexed
量子存在态 (Quantum Existence State)

一个基于Rust所开发的我的世界服务端。
参考了 FerrumC 等众多项目制作
> 目前版本对应:1.21.8

# 计划
下面根据我的世界java服网络数据包协议的进服顺序安排开发计划
- [ ] Handshake
    - [x] 跳转 Status 阶段
    - [x] 跳转 Login 阶段
    - [ ] 代理支持
- [x] StatusRequest
    - [x] Ping
- [ ] Login
    - [ ] Mojang 正版验证
    - [ ] 解压缩
    - [ ] 加解密
    - [x] 离线模式登录
    - [x] 跳转 Configuration 阶段
    - [x] 断开连接数据包
- [ ] Configuration
    - [x] 收发 Plugin_Message 事件
    - [ ] RegistryData 数据包支持
    - [ ] 注册表
    - [ ] forge 支持
    - [ ] geyser 支持

其他计划暂未制定