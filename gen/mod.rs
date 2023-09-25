pub mod fscp;
pub fn register_types(registry: &mut protokit::textformat::reflect::Registry) {
    fscp::register_types(registry);
}
