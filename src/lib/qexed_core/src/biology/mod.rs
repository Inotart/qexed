pub trait Biology {
    // 生物名(含命名空间)
    const NAME: &'static str;
    // 生物介绍(含命名空间)
    const DESCRIPTION: &'static str;
    // 是否默认显示名字
    const SHOW_NAME: bool = false;
    // 是否隐藏血量(如果是,则发送数据包给客户端时的血量为固定值99999999)
    const HIDE_HEALTH: bool = false;
    // 默认攻击力
    const DEFAULT_ATTACK_DAMAGE: f32 = 1.0;
    // 默认防御力
    const DEFAULT_ARMOR: f32 = 0.0;
    // 默认最大生命值
    const DEFAULT_MAX_HEALTH: f32 = 20.0;
    // 默认是否可以被玩家骑乘
    const DEFAULT_CAN_BE_RIDDEN: bool = false;
    // 默认是否可以被玩家驯服
    const DEFAULT_CAN_BE_TAMED: bool = false;
    // 默认是否可用移动
    const DEFAULT_CAN_MOVE: bool = true;
    // 默认是否可以游泳
    const DEFAULT_CAN_SWIM: bool = false;
    // 默认是否可以飞行
    const DEFAULT_CAN_FLY: bool = false;
    // 默认移动速度
    const DEFAULT_MOVEMENT_SPEED: f32 = 0.0;
    // 默认游泳速度
    const DEFAULT_SWIMMING_SPEED: f32 = 0.0;
    // 默认飞行速度
    const DEFAULT_FLYING_SPEED: f32 = 0.0;

    // ======== 函数 ========
    // 获取生物的唯一ID(若未设置,则返回-1)
    fn entity_id(&self) -> i32;
    // 设置生物唯一ID
    fn set_entity_id(&mut self, id: i32);
    // 设置显示的生物名
    fn set_display_name(&mut self, name: String);
    // 设置是否显示名字
    fn set_show_name(&mut self, show: bool);
    // 设置生物攻击力
    fn set_attack_damage(&mut self, damage: f32);
    // 设置生物防御力
    fn set_armor(&mut self, armor: f32);
    // 设置生物最大生命值
    fn set_max_health(&mut self, health: f32);
    // 设置生物当前生命值
    fn set_health(&mut self, health: f32);
    
}

pub mod player;