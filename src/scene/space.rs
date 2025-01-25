pub mod draw;
mod unit;
use glam::{IVec2, Vec2};
use ready_paint::scene::{return_res, Ready};
use std::{collections::HashMap, marker::PhantomData};
use unit::{IndexGrid, PosString};

#[derive(Default)]
pub struct CollisionMarker;
#[derive(Default)]
pub struct ClusteringMarker;
pub type Collision = SpaceMap<CollisionMarker>;
pub type Clustering = SpaceMap<ClusteringMarker>;
#[derive(Default)]
pub struct Space {
    // 如果多个空间cellsize去处理不同的大小的空间划分
    // 每次的update的hash取值也是可以在同一个大对象处理
    pub maps: Box<(Collision, Clustering)>,
}

#[derive(Default)]
pub struct SpaceMap<T> {
    cell_size: Vec2,
    map: HashMap<String, IndexGrid>,
    _marker: PhantomData<T>,
}

impl<T> SpaceMap<T> {
    fn new(cell_size: Vec2) -> Self {
        Self {
            cell_size,
            map: HashMap::new(),
            _marker: std::marker::PhantomData,
        }
    }
    pub fn clear(&mut self) {
        self.map.clear();
    }
    pub fn insert(&mut self, entity_id: u32, position: Vec2) {
        let cell_pos = self.get_cell_index(position);
        let key = PosString::from(cell_pos).value;
        self.map
            .entry(key)
            .or_insert(IndexGrid::new())
            .insert(entity_id);
    }

    fn get_index_grid_by_pos(&self, grid_pos: &IVec2) -> Option<&IndexGrid> {
        self.map.get(&PosString::from(grid_pos.clone()).value)
    }

    /// 查询某个位置的cell
    pub fn query(&self, entity_pos: Vec2) -> Option<&IndexGrid> {
        let index_pos = self.get_cell_index(entity_pos);
        self.map.get(&PosString::from(index_pos).value)
    }

    /// 根据位置返回cell的索引
    fn get_cell_index(&self, position: Vec2) -> IVec2 {
        IVec2::new(
            (position.x / self.cell_size.x).floor() as i32,
            (position.y / self.cell_size.y).floor() as i32,
        )
    }

    fn get_cell_center(&self, cell_pos: &IVec2) -> Vec2 {
        Vec2::new(
            cell_pos.x as f32 * self.cell_size.x + self.cell_size.x / 2.,
            cell_pos.y as f32 * self.cell_size.y + self.cell_size.y / 2.,
        )
    }
    // 根据目标位置和目标检测的半径，返回一个合适的cell_size，和目标位置正处于cell的中心点的划分空间
    pub fn adapte_target(&mut self, target: Vec2, radius: f32) {
        let rect = Vec2::new(radius * 2., radius * 2.);
        let first_cell_pos = self.get_cell_index(target);
        let cell_center = self.get_cell_center(&first_cell_pos);
        let diff = target - cell_center;
        let x_dir = if diff.x > 0. { -1 } else { 1 };
        let y_dir = if diff.y > 0. { -1 } else { 1 };
        let y_beside_cell = first_cell_pos + IVec2::new(0, y_dir);
        let x_beside_cell = first_cell_pos + IVec2::new(x_dir, 0);
        let xy_across_cell = first_cell_pos + IVec2::new(x_dir, y_dir);
        let total = [first_cell_pos, y_beside_cell, x_beside_cell, xy_across_cell]
            .iter()
            .fold(Vec::new(), |mut acc: Vec<u32>, grid_pos| {
                if let Some(grid) = self.get_index_grid_by_pos(grid_pos) {
                    acc.extend(grid.entity_ids.iter());
                }
                return acc;
            });

        [first_cell_pos, y_beside_cell, x_beside_cell, xy_across_cell]
            .iter()
            .for_each(|grid| {
                self.set_index_grid_entities(grid, total.clone());
            });
    }

    fn set_index_grid_entities(&mut self, grid_pos: &IVec2, entity_ids: Vec<u32>) {
        let key = PosString::from(grid_pos.clone()).value;
        self.map.insert(key, IndexGrid { entity_ids });
    }
}

impl Ready for Space {
    fn ready(
        &mut self,
        data: &mut ready_paint::scene::HashTypeId2Data,
        gfx: &ready_paint::gfx::Gfx,
    ) {
        return_res(
            data,
            Space {
                maps: Box::new((
                    Collision::new(Vec2::new(200., 200.)),
                    Clustering::new(Vec2::new(500., 500.)),
                )),
            },
        );
    }
}
