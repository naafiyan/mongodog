use mongodb::Database;

/// The Schemable trait provides the details associated with a data model struct
/// necessary to safely delete it and all the data an instance of this model owns.
pub trait Schemable {
    fn collection_name() -> &'static str;
    fn cascade_delete(&self); // TODO: add error handling/checking e.g. Result return type to
                              // TODO: determine if cascade_delete is necessary
}

/// Safe deletion for an object that implements the `Schemable` trait, where "safety"
/// is defined as the property that deleting a Schemable deletes all of the data it
/// exclusively owns, i.e. leaves no orphaned data.
pub fn safe_delete<T: Schemable>(to_delete: &T, db: &Database) {}
