use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub title: String,
    pub description: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiVersion {
    url: String,
    vers: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DataType {
    Const(String),
    GenericText(String),
    Integer(String),
    Date(String),
    URL(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataTypeWrapper {
    data: DataType,
    is_vec: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiQuery {
    api_version: ApiVersion,
    headers: Option<Vec<DataTypeWrapper>>,
    queries: Option<Vec<DataTypeWrapper>>,
    paths: Vec<DataTypeWrapper>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseSchema {
    schemas: Vec<ResponseSchema>,
    values: Vec<DataType>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
