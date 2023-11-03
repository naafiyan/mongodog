pub trait Schemable {
    fn collection_name(&self) -> &'static str;
    fn cascade_delete(&self); // TODO: add error handling/checking e.g. Result return type to
    // TODO: determine if cascade_delete is necessary
}

pub fn safe_delete<T: Schemable>(to_delete : &T) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: handle safe deletion 
    std::result::Result::Ok(()) // TODO: remove placeholder return
}
