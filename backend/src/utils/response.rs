use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SuccessResponse<T> 
where
    T: Serialize,
{
    pub status: String,
    pub data: T,
}

impl<T> SuccessResponse<T> 
where
    T: Serialize,
{
    pub fn new(data: T) -> Self {
        Self {
            status: "success".into(),
            data,
        }
    }
}