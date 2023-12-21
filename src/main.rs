use std::collections::HashMap;
use futures::future::LocalBoxFuture;
use std::pin::Pin;
use futures::future::FutureExt;
use serde::{Deserialize, Serialize};

type BoxedResult<'a, T:Serialize + Deserialize<'a>> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
type CalcFn<'a, T:Serialize + Deserialize<'a>> = Box<dyn Fn(T,T) -> LocalBoxFuture<'static, BoxedResult<'static, T>>>;

async fn add(a: i32, b: i32) -> BoxedResult<'static, i32> {
    Ok(a + b)
}

async fn sub(a: i32, b: i32) -> BoxedResult<'static, i32> {
    Ok(a - b)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut map: HashMap<&str, CalcFn<i32>> = Default::default();
    map.insert("add", Box::new(|a, b| add(a, b).boxed()));
    map.insert("sub", Box::new(|a, b| sub(a, b).boxed()));
    // please handle this unwrap properly in your code
    let result = map.get("add").unwrap()(2, 3).await;
    println!("result: {:?}", result);
    let result = map.get("sub").unwrap()(2, 3).await;
    println!("result: {:?}", result);
    Ok(())
}