
pub enum NetworkResult<T> {
    Ok(T),
    Err(ToRetry),
}

pub struct ToRetry {
    pub url: String,
    pub error: String,
    pub wait: u64, // ms
}