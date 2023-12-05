use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct IpInformation {
    #[serde(rename = "accuracyRadius")]
    pub accuracy_radius: i32,
    pub method: String,
    #[serde(rename = "queriesLeft")]
    pub queries_left: i32,
    pub city: String,
    pub ip: String,
    pub isp: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "timeZone")]
    pub time_zone: String,
    #[serde(rename = "lastAccess")]
    pub last_access: i64,
    pub network: String,
    pub proxy: bool,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    pub success: bool,
    pub organization: String,
    pub cached: bool,
    #[serde(rename = "countryName")]
    pub country_name: String,
    pub asn: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FailedResponse {
    pub success: bool,
    pub reason: String
}