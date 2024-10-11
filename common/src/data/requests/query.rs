use crate::{Config, Point};
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use actix_web::HttpResponse;
#[cfg(not(target_arch = "wasm32"))]
use redis::{FromRedisValue, ToRedisArgs};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct Points(pub Vec<Point>);

#[cfg(not(target_arch = "wasm32"))]
impl FromRedisValue for Points {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        // stored value is a BulkString, containing serialized JSON
        match *v {
            redis::Value::BulkString(ref bytes) => {
                let json_str = core::str::from_utf8(bytes).map_err(|e| {
                    redis::RedisError::from((
                        redis::ErrorKind::TypeError,
                        "Invalid UTF-8 string",
                        e.to_string(),
                    ))
                })?;

                // deserialize the JSON string into a Vec<Point>
                let points: Vec<Point> = serde_json::from_str(json_str).map_err(|e| {
                    redis::RedisError::from((
                        redis::ErrorKind::TypeError,
                        "Failed to deserialize into Vec<Point>",
                        e.to_string(),
                    ))
                })?;

                Ok(Points(points))
            }
            redis::Value::Nil => Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Key not found",
            ))),
            _ => Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "Unexpected Redis value type",
            ))),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ToRedisArgs for Points {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        // serialize the Vec<Point> into a JSON string
        let json = serde_json::to_string(&self.0).expect("Failed to serialize Points");

        // write to redis
        out.write_arg(json.as_bytes());
    }
}

pub trait QueryResponseTypes {}

#[derive(Deserialize, Serialize)]
pub struct QueryResponse<T: QueryResponseTypes> {
    response: T,
}

impl<T: QueryResponseTypes + Serialize> QueryResponse<T> {
    pub fn new(response: T) -> Self {
        Self { response }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn response(self) -> T {
        self.response
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self)
    }
}

impl QueryResponseTypes for String {}
impl QueryResponseTypes for Points {}
impl QueryResponseTypes for Config {}
