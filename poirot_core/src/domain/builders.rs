pub trait PoirotBuilder: Default {
    type PoirotTarget;
    fn build(&self) -> Result<Self::PoirotTarget, Box<dyn std::error::Error>>;
    fn new() -> Self {
        Self::default()
    }
}
