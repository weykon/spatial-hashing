pub mod draw;
mod unit;
use glam::{IVec2, Vec2};
use ready_paint::scene::{return_res, Ready};
use std::{
    collections::HashMap,
    fmt::{write, Display},
    marker::PhantomData,
};
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
    border_layer_map: Option<HashMap<String, IndexGrid>>,
    border_line_width: f32,
    x_entry: f32,
    y_entry: f32,
}

#[derive(Debug)]
pub enum BorderDir {
    LT,
    RT,
    LB,
    RB,
}
impl std::fmt::Display for BorderDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            BorderDir::LT => "LT".to_owned(),
            BorderDir::RT => "RT".to_owned(),
            BorderDir::LB => "LB".to_owned(),
            BorderDir::RB => "RB".to_owned(),
        };
        write!(f, "{}", text)
    }
}

#[test]
fn display_right() {
    assert_eq!(BorderDir::LB.to_string(), "LB");
}

impl<T> SpaceMap<T> {
    fn new(cell_size: Vec2) -> Self {
        Self {
            cell_size,
            map: HashMap::new(),
            _marker: std::marker::PhantomData,
            border_layer_map: None,
            border_line_width: 0.,
            x_entry: 0.,
            y_entry: 0.,
        }
    }
    pub fn clear(&mut self) {
        self.map.clear();
    }
    // fn check_close_border(
    //     &self,
    //     entity_pos: Vec2,
    //     border_map: &mut HashMap<String, IndexGrid>,
    //     cell_center: Vec2,
    // ) -> Option<IndexGrid> {
    //     border_map.entry(key)
    // }
    pub fn insert(&mut self, entity_id: u32, position: Vec2) {
        let cell_pos = self.get_cell_index(position);
        let key = PosString::from(cell_pos).value;
        let cell_center = &self.get_cell_center(&cell_pos);
        if let Some(border_map) = self.border_layer_map.as_mut() {
            // if let Some(border_grid) = self.check_close_border(position, border_map, cell_center) {}
            let dis = position - cell_center;
            if dis.x.abs() > self.x_entry && dis.y.abs() > self.y_entry {
                let border_dir = if dis.x > 0. && dis.y > 0. {
                    BorderDir::RT
                } else if dis.x < 0. && dis.y < 0. {
                    BorderDir::LB
                } else if dis.x < 0. && dis.y > 0. {
                    BorderDir::LT
                } else {
                    BorderDir::RB
                };
                let pos_string = PosString::from((cell_pos, border_dir));
                border_map
                    .entry(pos_string.value)
                    .or_insert(IndexGrid::new())
                    .insert(entity_id);
            }
        }
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

    fn set_index_grid_entities(&mut self, grid_pos: &IVec2, entity_ids: Vec<u32>) {
        let key = PosString::from(grid_pos.clone()).value;
        self.map.insert(key, IndexGrid { entity_ids });
    }

    // add border layer
    // the obj radius and distance for calculating to 2 * r + dis,
    // keep a full enough distance to
    fn with_border_layer(&mut self, object_radius: f32, object_center_separate_dis: f32) {
        let border_line_width = object_center_separate_dis + 2. * object_radius;
        self.border_layer_map = Some(HashMap::new());
        self.border_line_width = border_line_width;
        self.x_entry = self.cell_size.x - border_line_width;
        self.y_entry = self.cell_size.y - border_line_width;
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
