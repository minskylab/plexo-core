#[derive(Debug, seaography::macros::QueryRoot)]
#[seaography(entity = "crate::entities::member")]
#[seaography(entity = "crate::entities::project")]
#[seaography(entity = "crate::entities::task")]
pub struct QueryRoot;
