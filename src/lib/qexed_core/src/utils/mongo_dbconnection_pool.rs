use anyhow::{Context, Result};
use mongodb::{bson::doc, options::ClientOptions, Client};
use qexed_config::get_global_config;

#[derive(Debug)]
pub struct MongoDBConnectionPool {
    client: Client,
    default_db: String,
}

impl MongoDBConnectionPool {
    /// 创建新的 MongoDB 连接池
    pub async fn new() -> Result<Self> {
        let config = get_global_config()?;
        let mongodb_config = &config.database.mongodb;
        
        // 1. 解析连接字符串
        let mut client_options = ClientOptions::parse(&mongodb_config.connection_string)
            .await
            .context("Failed to parse MongoDB connection string")?;
        
        // 2. 配置连接池参数
        client_options.min_pool_size = Some(mongodb_config.min_pool_size);
        client_options.max_pool_size = Some(mongodb_config.max_pool_size);
        // client_options.max_idle_time = mongodb_config.max_idle_time.map(Duration::from_secs);
        // client_options.connect_timeout = mongodb_config.connect_timeout.map(Duration::from_secs);
        // client_options.server_selection_timeout = mongodb_config.server_selection_timeout.map(Duration::from_secs);
        
        // 3. 创建带连接池的客户端
        let client = Client::with_options(client_options)
            .context("Failed to create MongoDB client with options")?;
        
        // 4. 验证连接
        Self::check_connection(&client).await?;
        
        Ok(Self {
            client,
            default_db: mongodb_config.default_database.clone(),
        })
    }
    
    /// 获取默认数据库
    pub fn default_database(&self) -> &str {
        &self.default_db
    }
    
    /// 获取数据库实例
    pub fn database(&self, name: &str) -> mongodb::Database {
        self.client.database(name)
    }
    
    /// 获取默认数据库实例
    pub fn default_db(&self) -> mongodb::Database {
        self.client.database(&self.default_db)
    }
    
    /// 检查连接是否有效
    async fn check_connection(client: &Client) -> Result<()> {
        // 尝试列出数据库以验证连接
        client.list_database_names().await
            .context("Failed to connect to MongoDB server")?;
        
        // 检查集群状态
        let db = client.database("admin");
        let server_status_doc = db.run_command(mongodb::bson::doc! { "serverStatus": 1 }).await
            .context("Failed to get MongoDB server status")?;

        let host = server_status_doc.get_str("host").unwrap_or("unknown");
        let version = server_status_doc.get_str("version").unwrap_or("unknown");

        log::info!(
            "Connected to MongoDB cluster: {} (v{})", 
            host,
            version
        );
        
        Ok(())
    }
    
    // MongoDB Rust driver does not expose pool status directly.
    // You may implement custom metrics or logging if needed.
    
    /// 健康检查
    pub async fn health_check(&self) -> Result<()> {
        // 执行简单的 ping 操作
        let db = self.default_db();
        let command = doc! {"ping": 1};
        db.run_command(command).await
            .context("MongoDB health check failed")?;
        Ok(())
    }
}

// 使用示例
pub async fn use_mongodb_pool() -> Result<()> {
    // 创建连接池
    let pool = MongoDBConnectionPool::new().await?;
    
    // 打印连接池状态（MongoDB Rust driver does not expose pool status directly）
    println!("MongoDB connection pool created.");
    pool.health_check().await?;
    
    // 获取默认数据库
    let db = pool.default_db();
    
    // 执行数据库操作...
    let collection = db.collection::<mongodb::bson::Document>("users");
    let count = collection.count_documents(doc! {}).await?;
    println!("Total users: {}", count);
    
    Ok(())
}