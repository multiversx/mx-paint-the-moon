use actix_web::HttpResponse;
use common::{Point, QueryResponse, QueryResponseTypes};
use redis::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct PointsNonWasm(pub Vec<Point>);

impl FromRedisValue for PointsNonWasm {
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

                Ok(PointsNonWasm(points))
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

impl ToRedisArgs for PointsNonWasm {
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

pub struct QueryResponseNonWasm<T: QueryResponseTypes>(pub QueryResponse<T>);

impl<T: QueryResponseTypes + Serialize> QueryResponseNonWasm<T> {
    pub fn new(response: T) -> Self {
        Self(QueryResponse::new(response))
    }

    pub fn response(&self) -> HttpResponse {
        HttpResponse::Ok().json(&self.0)
    }
}

impl QueryResponseTypes for PointsNonWasm {}
