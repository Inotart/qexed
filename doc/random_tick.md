# 随机刻
在qexed中,我们丢弃掉随机刻这个机制。因为随机刻每时每刻都需要进行运算。
相对的,我们使用异步事件,将各自事件延迟执行。以事先确认好了，到时候再执行.
但是这个时间我们是不确定的,如果按照原版那样子随机那样子会加大计算量,为了极致优化我们将刻意的使用一个函数随机
因此我们将改成使用一个固定的函数来生成随机刻

```rust
/// 生成符合随机刻特性的时间（秒）
/// - `u`: [0,1] 区间的均匀分布随机数
/// - `n`: 每刻更新的方块数（默认3）
pub fn random_tick_time(u: f64, n: usize) -> f64 {
    // 核心参数（基于中位数47.3秒和平均数68.27秒）
    let lambda = 1.0 / 68.27; // 指数分布参数
    let median = 47.3;        // 中位数
    let min_bound = 20.0;     // 最小时间
    let max_bound = 120.0;    // 最大时间

    // 1. 方块数量修正因子（n=3时为基准值1）
    let n_factor = 3.0 / n as f64;

    // 2. 基础指数分布变换
    let mut t = -f64::ln(1.0 - u) / lambda * n_factor;

    // 3. 中位数对齐修正
    if u < 0.5 {
        // 前半段：压缩短间隔（加速）
        t = median * (t / median).powf(0.8);
    } else {
        // 后半段：拉伸长间隔（减速）
        t = median + (max_bound - median) * ((t - median) / (max_bound - median)).powf(1.2);
    }

    // 4. 边界约束
    t.clamp(min_bound, max_bound)
}
```

其中中位数和平均数数据来自wiki[刻](https://zh.minecraft.wiki/w/%E5%88%BB?variant=zh-cn#%E9%9A%8F%E6%9C%BA%E5%88%BB)
