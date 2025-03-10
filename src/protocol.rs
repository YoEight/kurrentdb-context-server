use serde::Deserialize;

#[derive(Deserialize)]
pub struct McpRequest {
    pub jsonrpc: String,
    pub id: u64,
    pub method: String,
    pub params: serde_json::Value,
}

#[derive(Deserialize)]
pub struct McpResponse<A> {
    pub jsonrpc: String,
    pub id: u64,
    pub result: A,
}
