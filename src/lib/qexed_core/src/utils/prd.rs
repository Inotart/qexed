use rand::Rng;
use std::fmt;

/// PRD 状态机结构体（使用二次函数调整概率）
pub struct QuadraticPRD {
    initial_probability: f64, // 初始概率
    current_probability: f64, // 当前概率
    max_attempts: u32,        // 最大尝试次数
    attempts: u32,            // 当前尝试次数
    success_count: u32,       // 成功次数
    quadratic_factor: f64,    // 二次函数系数，控制概率增长曲线
}

impl QuadraticPRD {
    /// 创建一个新的 PRD 状态机
    pub fn new(probability: f64, max_attempts: u32, quadratic_factor: f64) -> Result<Self, &'static str> {
        if probability <= 0.0 || probability > 1.0 {
            return Err("概率必须在 (0, 1] 范围内");
        }
        
        if quadratic_factor <= 0.0 {
            return Err("二次函数系数必须大于 0");
        }
        
        Ok(Self {
            initial_probability: probability,
            current_probability: probability,
            max_attempts,
            attempts: 0,
            success_count: 0,
            quadratic_factor,
        })
    }
    
    /// 重置状态机到初始状态
    pub fn reset(&mut self) {
        self.current_probability = self.initial_probability;
        self.attempts = 0;
    }
    
    /// 使用二次函数计算下一次尝试的概率
    fn calculate_next_probability(&self) -> f64 {
        // 使用二次函数: p(n) = p0 + a*n^2
        // 其中 p0 是初始概率，a 是二次函数系数，n 是尝试次数
        let next_prob = self.initial_probability + 
                       self.quadratic_factor * (self.attempts as f64).powi(2);
        
        // 确保概率不超过 1.0
        next_prob.min(1.0)
    }
    
    /// 尝试一次并返回是否成功
    pub fn try_once(&mut self) -> bool {
        if self.attempts >= self.max_attempts {
            // 达到最大尝试次数，强制成功并重置
            self.success_count += 1;
            self.reset();
            return true;
        }
        
        let mut rng = rand::rng();
        let random_value: f64 = rng.random();
        
        // 如果随机值小于当前概率，则成功
        if random_value < self.current_probability {
            self.success_count += 1;
            self.reset();
            true
        } else {
            // 失败时使用二次函数计算下一次概率
            self.current_probability = self.calculate_next_probability();
            self.attempts += 1;
            false
        }
    }
    
    /// 获取当前概率
    pub fn current_probability(&self) -> f64 {
        self.current_probability
    }
    
    /// 获取成功次数
    pub fn success_count(&self) -> u32 {
        self.success_count
    }
    
    /// 获取尝试次数
    pub fn attempts(&self) -> u32 {
        self.attempts
    }
    
    /// 获取初始概率
    pub fn initial_probability(&self) -> f64 {
        self.initial_probability
    }
}

// 为 QuadraticPRD 实现 Display trait 以便打印状态
impl fmt::Display for QuadraticPRD {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "QuadraticPRD {{ initial_probability: {:.3}, current_probability: {:.3}, attempts: {}, success_count: {} }}",
            self.initial_probability, self.current_probability, self.attempts, self.success_count
        )
    }
}
#[test]
// 测试函数
fn main() {
    // 创建一个初始概率为 0.2 的 PRD 状态机，最大尝试次数为 60，二次函数系数为 0.001
    let mut prd = QuadraticPRD::new(0.2, 60, 0.001).unwrap();
    
    println!("初始状态: {}", prd);
    let mut trues = 0;
    let total_tries = 10000;
    
    // 模拟多次尝试
    for i in 0..total_tries {
        let success = prd.try_once();
        if success {
            trues += 1;
        }
        
        // 每1000次打印一次状态
        if (i + 1) % 1000 == 0 {
            println!("尝试 {}: 成功率 {:.2}%", i + 1, (trues as f64 / (i + 1) as f64) * 100.0);
        }
    }
    
    println!("\n最终统计:");
    println!("总尝试次数: {}", total_tries);
    println!("成功次数: {}", trues);
    println!("实际成功率: {:.2}%", (trues as f64 / total_tries as f64) * 100.0);
}