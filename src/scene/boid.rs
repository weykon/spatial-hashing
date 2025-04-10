use std::ops::{Deref, DerefMut};

use glam::Vec2;
use rand::Rng;
use ready_paint::{
    multi::{refs_muts, Mut, Ref},
    scene::{get_res, get_res_mut, return_res, Ready, Update},
};

use super::{config::BoidConfig, entity::Entity, space::Space};

#[derive(Default)]
pub struct Boid {
    masses: Vec<f32>,
    accs: Vec<Vec2>,
    velocities: Vec<[f32; 2]>,
    target: Vec2,
}

impl Ready for Boid {
    fn ready(&mut self, data: &mut ready_paint::scene::HashTypeId2Data, _: &ready_paint::gfx::Gfx) {
        let entity = get_res_mut::<Entity>(data);
        let base_acc: Vec2 = Vec2::ZERO;
        let instance_collect = entity.instance_collect.as_ref().unwrap();
        let counts = instance_collect.len();
        let radius = instance_collect.clone();
        let masses: Vec<f32> = radius.iter().map(|i| i.radius).collect();
        let accs: Vec<Vec2> = (0..counts).map(|_| base_acc).collect();
        let velocities: Vec<[f32; 2]> = instance_collect
            .clone()
            .iter()
            .map(|i| i.velocity)
            .collect();
        return_res(
            data,
            Boid {
                masses,
                accs,
                velocities,
                target: Vec2::new(400., 400.),
            },
        );
    }
}

impl Update for Boid {
    fn update(data: &mut ready_paint::scene::HashTypeId2Data, gfx: &ready_paint::gfx::Gfx) {
        let dt = gfx.delta_time;
        let config = gfx.surface_config.as_ref().unwrap();
        let (entity, boid, space, boid_config) =
            refs_muts::<(Mut<Entity>, Mut<Boid>, Mut<Space>, Ref<BoidConfig>)>(data);
        let entity_poses = entity
            .instance_collect
            .as_mut()
            .unwrap()
            .iter_mut()
            .map(|i| Vec2::from_array(i.position))
            .collect::<Vec<Vec2>>();

        let collision_space = &mut space.maps.0;
        let clustering_space = &mut space.maps.1;

        // 更新空间
        collision_space.clear();
        clustering_space.clear();

        for (i, pos) in entity_poses.iter().enumerate() {
            collision_space.insert(i as u32, *pos);
            clustering_space.insert(i as u32, *pos);
        }


        // 目标位置的更新逻辑

        // 实体insert到哈希空间

        let velocities = boid.velocities.deref_mut();
        let accs = boid.accs.deref_mut();
        let masses = boid.masses.deref_mut();

        for (i, (pos, (acc, (mass, velocity)))) in entity_poses
            .iter()
            .zip(accs.iter().zip(masses.iter().zip(velocities.iter())))
            .enumerate()
        {
            let current_pos = pos;
            let mut separation = Vec2::ZERO;
            let mut alignment = Vec2::ZERO;
            let mut cohesion = Vec2::ZERO;
            let mut neighbors = 0;

            // 分离: 避免碰撞
            let index_grid = collision_space.query(*current_pos).unwrap();


            // 对齐和内聚: 使用更大的范围
            let index_grid = clustering_space.query(*current_pos).unwrap();
            for neighbor_id in index_grid.entity_ids.iter() {
                if *neighbor_id as usize == i {
                    continue;
                }
                let neighbor_pos = entity_poses[*neighbor_id as usize];
                let diff = current_pos - neighbor_pos;
                let dist = diff.length();

                // 对齐: 速度方向一致
                if dist < boid_config.alignment_max_radius
                    && dist > boid_config.alignment_min_radius
                {
                    let neighbor_vel = Vec2::from_slice(&velocities[*neighbor_id as usize]);
                    alignment += neighbor_vel;
                }

                // 内聚: 向群体中心移动
                if dist < boid_config.cohesion_radius && dist > 0.0 {
                    cohesion += neighbor_pos;
                    neighbors += 1;
                }
            }

            if neighbors > 0 {
                alignment /= neighbors as f32;
                cohesion = cohesion / neighbors as f32 - current_pos;
            }

            // 计算期望的速度方向， 三力合一，加一个目标力
            let desired_direction = {
                let mut dir = Vec2::ZERO;

                // 添加各种力的影响
                if separation.length() > 0.0 {
                    dir += separation.normalize() * boid_config.separation_weight;
                    // 分离力
                }
                if alignment.length() > 0.0 {
                    dir += alignment.normalize() * boid_config.alignment_weight;
                    // 对齐力
                }
                if cohesion.length() > 0.0 {
                    dir += cohesion.normalize() * boid_config.cohesion_weight; // 内聚力
                }

                // 目标力
                let to_target = boid.target - *current_pos;
                if to_target.length() > 0.0 {
                    let mut target_influence =
                        (to_target.length() / boid_config.target_influence_scale).min(1.2);
                    if to_target.length() < boid_config.target_min_distance {
                        target_influence = 0.05;
                    }
                    dir += to_target.normalize() * target_influence * boid_config.target_weight;
                    // 增加目标影响
                }

                if dir.length() > 0.0 {
                    dir.normalize()
                } else {
                    Vec2::from_slice(velocity).normalize()
                }
            };

            // 获取当前速度方向
            let current_direction = Vec2::from_slice(velocity).normalize();

            // 计算转向力
            let steer = {
                let dot = current_direction.dot(desired_direction);
                let angle = dot.clamp(-1.0, 1.0).acos(); // 防止数值误差

                // 使用 slerp 进行平滑转向
                let angle_factor = (angle / std::f32::consts::PI).min(1.0);
                let t = boid_config.steer_strength
                    * (1.0 - angle_factor * boid_config.steer_angle_factor);
                let steer_dir = if angle < 0.001 {
                    desired_direction
                } else {
                    let sin_angle = angle.sin();
                    if sin_angle < 0.001 {
                        desired_direction
                    } else {
                        let s0 = (angle * (1.0 - t)).sin() / sin_angle;
                        let s1 = (angle * t).sin() / sin_angle;
                        (current_direction * s0 + desired_direction * s1).normalize()
                    }
                };

                steer_dir * boid_config.max_steer_force
            };

            // 应用转向力来更新速度
            let mut new_velocity =
                Vec2::from_slice(velocity) + steer * dt * boid_config.base_acc_scale;

            // 速度限制
            let speed = new_velocity.length();
            if speed < boid_config.min_speed {
                new_velocity = new_velocity.normalize() * boid_config.min_speed;
            } else if speed > boid_config.max_speed {
                new_velocity = new_velocity.normalize() * boid_config.max_speed;
            }

            // 更新实例数据
            let instance = &mut entity.instance_collect.as_mut().unwrap()[i];
            instance.velocity = new_velocity.to_array();
            instance.position[0] += new_velocity.x * dt;
            instance.position[1] += new_velocity.y * dt;
            // 边界处理
            let margin = boid_config.boundary_margin;
            let width = config.width as f32;
            let height = config.height as f32;

            if instance.position[0] < -margin {
                instance.position[0] = width + margin;
            }
            if instance.position[0] > width + margin {
                instance.position[0] = -margin;
            }
            if instance.position[1] < -margin {
                instance.position[1] = height + margin;
            }
            if instance.position[1] > height + margin {
                instance.position[1] = -margin;
            }
        }
        let instance_buffer = entity.instance_buffer.as_mut().unwrap();
        let data_bytes = bytemuck::cast_slice(entity.instance_collect.as_ref().unwrap().as_slice());
        gfx.queue.write_buffer(instance_buffer, 0, data_bytes);
    }
}

impl Boid {
    pub fn set_target(&mut self, target: Vec2) {
        self.target = target;
    }
}
mod entry;
