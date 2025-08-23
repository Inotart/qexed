use rand::{Rng, rng};
use rayon::prelude::*;
// 常量预计算（避免重复计算）
const LAMBDA: f64 = 1.0 / 68.27; // 1/68.27 ≈ 0.014646
const MEDIAN: f64 = 47.3;
const MIN_BOUND: f64 = 20.0;
const MAX_BOUND: f64 = 120.0;
const SCALE_FACTOR: f64 = 3.0; // 基准方块数因子
/// 生成符合随机刻特性的时间（秒）
/// - `u`: [0,1] 区间的均匀分布随机数
/// - `n`: 每刻更新的方块数（默认3）
#[inline]
pub fn random_tick_time(u: f64, n: usize) -> f64 {
    // 1. 方块数量修正因子（无分支计算）
    let n_factor = SCALE_FACTOR / n as f64;
    
    // 2. 基础指数变换（合并计算）
    let log_arg = 1.0 - u;
    let mut t = -log_arg.ln() * LAMBDA.recip() * n_factor; // 等价于 -ln(1-u)/λ * n_factor

    // 3. 中位数对齐修正（避免重复除法）
    if u < 0.5 {
        // 等价于 t = MEDIAN * (t / MEDIAN).powf(0.8)
        let ratio = t / MEDIAN;
        t = MEDIAN * ratio.powf(0.8); // 保留原始逻辑
    } else {
        // 等价于 t = MEDIAN + (MAX_BOUND - MEDIAN) * ((t - MEDIAN)/(MAX_BOUND - MEDIAN)).powf(1.2)
        let norm_range = MAX_BOUND - MEDIAN;
        let norm_t = (t - MEDIAN) / norm_range;
        t = MEDIAN + norm_range * norm_t.powf(1.2);
    }

    // 4. 边界约束（无分支 clamp）
    t.max(MIN_BOUND).min(MAX_BOUND)
}
/// 生成预计算的随机刻时间列表
/// - `x`: 需要生成的样本数量
/// - `n`: 每刻更新的方块数
/// 
/// 返回: Vec<f64> 包含x个随机刻时间值的列表
pub fn generate_tick_times(x: usize, n: usize) -> Vec<f64> {
    // 使用并行迭代器高效生成随机数
    (0..x)
        .into_par_iter()  // 开启并行处理
        .map_init(
            || rng(),  // 每个线程初始化独立RNG
            |rng, _| {
                // 生成[0,1)区间的均匀分布随机数
                let u = rng.random_range(0.0..1.0);
                // 计算随机刻时间
                random_tick_time(u, n)
            }
        )
        .collect()  // 收集结果到Vec
}
/// 测试函数：生成样本并统计指标
#[test]
fn test_random_tick_distribution() {
    let n = 3; // 默认方块数
    let sample_size = 100_000;
    let mut samples = Vec::with_capacity(sample_size);

    // 生成样本
    for _ in 0..sample_size {
        let u = rand::random::<f64>(); // 依赖 rand 库生成随机数
        samples.push(random_tick_time(u, n));
    }

    // 计算统计指标
    let median = samples[sample_size / 2];
    let mean: f64 = samples.iter().sum::<f64>() / sample_size as f64;
    let min = samples.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max = samples.iter().fold(0.0f64, |a, &b| a.max(b));

    // 短间隔（<1秒）和长间隔（>300秒）概率
    let short_count = samples.iter().filter(|&&t| t < 1.0).count();
    let long_count = samples.iter().filter(|&&t| t > 300.0).count();

    println!("中位数: {:.1}s (目标: 47.3s)", median);
    println!("平均数: {:.2}s (目标: 68.27s)", mean);
    println!("最小值: {:.1}s, 最大值: {:.1}s", min, max);
    println!("短间隔概率: {:.2}% (目标: 1.38%)", short_count as f64 / sample_size as f64 * 100.0);
    println!("长间隔概率: {:.2}% (目标: 1.23%)", long_count as f64 / sample_size as f64 * 100.0);
}