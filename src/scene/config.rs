use glam::Vec2;
use ready_paint::scene::Ready;

pub struct Config {
    pub max_entities: u32,
    pub entity_max_speed: f32,
}

impl Ready for Config {
    fn ready(
        &mut self,
        data: &mut ready_paint::scene::HashTypeId2Data,
        gfx: &ready_paint::gfx::Gfx,
    ) {
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_entities: 1000,
            entity_max_speed: 10.,
        }
    }
}

pub struct BoidConfig {
    // 基础运动参数
    pub base_acc_scale: f32,
    pub min_speed: f32,
    pub max_speed: f32,

    // 转向参数
    pub max_steer_force: f32,
    pub steer_strength: f32,
    pub steer_angle_factor: f32, // 用于角度衰减的系数

    // 力的权重
    pub separation_weight: f32,
    pub alignment_weight: f32,
    pub cohesion_weight: f32,
    pub target_weight: f32,

    // 感知范围
    pub separation_radius: f32,    // 分离力感知范围
    pub alignment_min_radius: f32, // 对齐力最小感知范围
    pub alignment_max_radius: f32, // 对齐力最大感知范围
    pub cohesion_radius: f32,      // 内聚力感知范围

    // 目标相关
    pub target_influence_scale: f32,   // 目标影响力的缩放因子
    pub target_min_distance: f32,      // 接近目标时降低影响的距离
    pub target_arrival_threshold: f32, // 判定到达目标的距离阈值

    // 边界参数
    pub boundary_margin: f32,
}

impl Ready for BoidConfig {
    fn ready(
        &mut self,
        data: &mut ready_paint::scene::HashTypeId2Data,
        gfx: &ready_paint::gfx::Gfx,
    ) {
    }
}
impl Default for BoidConfig {
    fn default() -> Self {
        Self {
            // 基础运动参数
            base_acc_scale: 40.0,
            min_speed: 10.0,
            max_speed: 350.0,

            // 转向参数
            max_steer_force: 70.0,
            steer_strength: 0.9,
            steer_angle_factor: 0.3,

            // 力的权重
            separation_weight: 0.4,
            alignment_weight: 1.4,
            cohesion_weight: 0.4,
            target_weight: 1.2,

            // 感知范围
            separation_radius: 10.0,
            alignment_min_radius: 20.0,
            alignment_max_radius: 80.0,
            cohesion_radius: 100.0,

            // 目标相关
            target_influence_scale: 30.0,
            target_min_distance: 80.0,
            target_arrival_threshold: 100.0,

            // 边界参数
            boundary_margin: 50.0,
        }
    }
}
