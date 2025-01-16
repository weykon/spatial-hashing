pub mod scene;
mod run;
fn main() {
    pollster::block_on(run::run());
}
