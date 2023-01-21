pub trait ByPK<T> {
    fn get_by_pk(&self) -> &str;
}

pub trait Where<T> {
    fn filter(&self, filter_name: &str, filter_value: &str) -> T;
}

pub trait Accessor<T, PK, WH>
where
    PK: ByPK<T>,
    WH: Where<T>,
{
    fn select(where_conditions: WH) -> Vec<T>;
    fn aggregate() -> Vec<T>;
    fn select_by_pk(pk: PK) -> T;
    fn insert(data: Vec<T>) -> Vec<T>;
    fn insert_one(data: T) -> T;
    fn update(data: Vec<T>) -> Vec<T>;
    fn update_one(data: T) -> T;
    fn update_by_pk(pk: PK) -> T;
    fn delete(data: Vec<T>) -> Vec<T>;
    fn delete_by_pk(pk: PK) -> T;
}

pub struct StringComparisonExp {
    pub _eq: Option<String>,
    pub _gt: Option<String>,
    pub _gte: Option<String>,
    pub _i_like: Option<String>,
    pub _in_: Option<Vec<String>>,
    pub _i_regex: Option<String>,
    pub _is_null: Option<bool>,
    pub _like: Option<String>,
    pub _lt: Option<String>,
    pub _lte: Option<String>,
    pub _neq: Option<String>,
    pub _ni_like: Option<String>,
    pub _nin: Option<Vec<String>>,
    pub _ni_regex: Option<String>,
    pub _n_like: Option<String>,
    pub _n_regex: Option<String>,
    pub _n_similar: Option<String>,
    pub _regex: Option<String>,
    pub _similar: Option<String>,
}
