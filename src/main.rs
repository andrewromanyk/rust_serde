use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};
use serde_json;
use serde_yaml::to_string as to_yaml;
use std::error::Error;
use std::fs::File;
use std::time::Duration;
use toml;
use toml::to_string as to_toml;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
    birthdate: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PublicTariff {
    id: u32,
    price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration, // Duration
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrivateTariff {
    client_price: u32,
    #[serde(with = "humantime_serde")]
    duration: Duration,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Stream {
    user_id: Uuid,
    is_private: bool,
    settings: u32,
    shard_url: Url, //url
    public_tariff: PublicTariff,
    private_tariff: PrivateTariff,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
struct Gift {
    id: u32,
    price: u32,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Debug {
    #[serde(with = "humantime_serde")]
    duration: Duration,
    at: DateTime<Utc>, //data
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
enum RequestType {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
}

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    #[serde(rename = "type")]
    request_type: RequestType,
    stream: Stream,
    gifts: Vec<Gift>,
    debug: Debug,
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    name: String,
    #[serde(
        serialize_with = "serialize_date",
        deserialize_with = "deserialize_date"
    )]
    date: String,
}

fn serialize_date<S: Serializer>(date: &str, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&format!("Date: {}", date))
}

fn deserialize_date<'a, D: Deserializer<'a>>(deserializer: D) -> Result<String, D::Error> {
    let data: &str = Deserialize::deserialize(deserializer)?;
    Ok(data.replace("Date:", ""))
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_1() {
        let file = File::open("request.json").unwrap();
        let request: Request = serde_json::from_reader(file).unwrap();

        assert_eq!(request.request_type, RequestType::Success);
        assert_eq!(
            request.stream.user_id,
            Uuid::from_str("8d234120-0bda-49b2-b7e0-fbd3912f6cbf").unwrap()
        );
        assert_eq!(
            request.gifts[0],
            Gift {
                id: 1,
                price: 2,
                description: "Gift 1".to_string()
            }
        )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // let user = User{
    //     name:"John".to_string(),
    //     email:"johndoe321@gmail.com".to_string(),
    //     birthdate:"5.06.97".to_string()
    // };
    //
    // let json = serde_json::to_string(&user)?;
    // println!("{}", json);
    //
    // let deser_user: User = serde_json::from_str(&json)?;
    // println!("{:?}", deser_user);

    // let file = File::open("request.json")?;
    // let request: Request = serde_json::from_reader(file)?;
    //
    // println!("{:#?}", request);
    //
    // println!("yaml: {}", to_yaml(&request)?);
    // println!("toml: {}", to_toml(&request)?);

    let event = Event {
        name: "Event 1".to_string(),
        date: "2020-01-01".to_string(),
    };

    let json = serde_json::to_string(&event).unwrap();
    println!("{}", json);

    Ok(())
}
