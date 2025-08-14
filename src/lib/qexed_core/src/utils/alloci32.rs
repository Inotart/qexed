use std::collections::BTreeMap;
use std::fmt;
pub trait ALLOC<T>: std::fmt::Debug {
    // 创建一个新的实例
    fn new() -> Self;
    // 获取值的方法
    fn get(&mut self) -> anyhow::Result<T>;
    // 删除值的方法
    fn delete(&mut self, v: T);
}

// 自定义错误类型
#[derive(Debug, PartialEq)]
pub enum AllocError {
    Exhausted, // 资源已耗尽
}
impl std::error::Error for AllocError {}
impl fmt::Display for AllocError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AllocError::Exhausted => write!(f, "Resource exhausted"),
        }
    }
}
#[derive(Debug)]
pub struct Alloci32 {
    // 按起始位置排序的区间树
    start_map: BTreeMap<i32, i32>, // start -> end
    // 按结束位置排序的区间树（用于快速合并）
    end_map: BTreeMap<i32, i32>, // end -> start
    // 缓存最小可用值（优化快速分配）
    min_value: Option<i32>,
    // 缓存下一个分配位置（减少树查找）
    next_alloc: Option<i32>,
}

impl Alloci32 {
    // 插入一个新区间（自动合并相邻区间）- 优化版本
    fn insert_interval(&mut self, mut start: i32, mut end: i32) {
        // 确保区间有效
        if start > end {
            return;
        }
        
        // 更新最小值缓存
        if self.min_value.map_or(true, |min| start < min) {
            self.min_value = Some(start);
        }
        
        // 更新分配缓存
        if self.next_alloc.map_or(true, |next| start < next) {
            self.next_alloc = Some(start);
        }

        // 优化：检查是否可以直接合并到前一个区间
        if let Some((&prev_end, &prev_start)) = self.end_map.range(..=start).next_back() {
            if prev_end == i32::MAX || prev_end + 1 >= start {
                start = prev_start.min(start);
                end = prev_end.max(end);
                self.remove_interval(prev_start, prev_end);
            }
        }

        // 优化：检查是否可以直接合并到后一个区间
        if let Some((&next_start, &next_end)) = self.start_map.range(start..).next() {
            if next_start <= end.saturating_add(1) {
                end = end.max(next_end);
                self.remove_interval(next_start, next_end);
            }
        }

        // 插入最终合并后的区间
        self.start_map.insert(start, end);
        self.end_map.insert(end, start);
    }

    fn remove_interval(&mut self, start: i32, end: i32) {
        self.start_map.remove(&start);
        self.end_map.remove(&end);
        
        // 如果移除的是最小值缓存，则清空缓存
        if self.min_value == Some(start) {
            self.min_value = None;
        }
        
        // 如果移除的是分配缓存，则清空缓存
        if self.next_alloc == Some(start) {
            self.next_alloc = None;
        }
    }

    // 分配一个值
    fn alloc_value(&mut self) -> Option<i32> {
        // 1. 首先尝试使用缓存的分配位置
        if let Some(next) = self.next_alloc {
            if let Some(&end) = self.start_map.get(&next) {
                let value = next;
                
                // 直接更新区间而不调用insert_interval
                if next == end {
                    self.remove_interval(next, end);
                } else {
                    let new_start = next + 1;
                    self.remove_interval(next, end);
                    self.start_map.insert(new_start, end);
                    self.end_map.insert(end, new_start);
                    
                    // 更新分配缓存
                    self.next_alloc = Some(new_start);
                }
                
                // 更新最小值缓存
                if self.min_value == Some(value) {
                    self.min_value = Some(value + 1);
                }
                
                return Some(value);
            }
        }
        
        // 2. 尝试使用缓存的最小值
        if let Some(min) = self.min_value {
            if let Some(&end) = self.start_map.get(&min) {
                let value = min;
                
                if min == end {
                    self.remove_interval(min, end);
                } else {
                    let new_start = min + 1;
                    self.remove_interval(min, end);
                    self.start_map.insert(new_start, end);
                    self.end_map.insert(end, new_start);
                    
                    // 更新分配缓存
                    self.next_alloc = Some(new_start);
                }
                
                // 更新最小值缓存
                self.min_value = Some(value + 1);
                
                return Some(value);
            }
        }
        
        // 3. 最后使用BTreeMap的最小键
        if let Some((&start, &end)) = self.start_map.first_key_value() {
            let value = start;
            
            if start == end {
                self.remove_interval(start, end);
            } else {
                let new_start = start + 1;
                self.remove_interval(start, end);
                self.start_map.insert(new_start, end);
                self.end_map.insert(end, new_start);
                
                // 更新分配缓存
                self.next_alloc = Some(new_start);
            }
            
            // 更新最小值缓存
            self.min_value = Some(value + 1);
            
            return Some(value);
        }
        
        None
    }
}

impl ALLOC<i32> for Alloci32 {
    fn new() -> Self {
        let mut alloc = Self {
            start_map: BTreeMap::new(),
            end_map: BTreeMap::new(),
            min_value: Some(0),
            next_alloc: Some(0),
        };
        // 初始化整个i32空间为可用
        alloc.start_map.insert(0, i32::MAX);
        alloc.end_map.insert(i32::MAX, 0);
        alloc
    }

    fn get(&mut self) -> anyhow::Result<i32> {
        self.alloc_value()
            .ok_or(AllocError::Exhausted.into())
    }

    fn delete(&mut self, value: i32) {
        // 插入单个值作为一个区间
        self.insert_interval(value, value);
    }
}