#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8000")?;
    hc.do_post("/user", json!({
        "username": "demo1",
        "phone": "17777"
    })).await?.print().await?;
    hc.do_get("/users").await?.print().await?;
    hc.do_get("/user/1000").await?.print().await?;
    hc.do_put("/user/1000", json!({
        "username": "demo3",
        "phone": "18888"
    })).await?.print().await?;
    hc.do_delete("/user/1000").await?.print().await?;

	Ok(())
}