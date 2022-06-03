pub trait Character {
    fn name(&self) -> &'static str;
}

struct Mario;
struct Luigi;

impl Character for Mario {
    fn name(&self) -> &'static str {
        "Mario"
    }
}

impl Character for Luigi {
    fn name(&self) -> &'static str {
        "Luigi"
    }
}