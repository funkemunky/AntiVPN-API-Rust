mod util;

#[cfg(feature = "async")]
mod apiasync;

use util::structs::{IpInformation, FailedResponse};
use serde_json::Value;

pub fn get_ip_info(ip: String) -> Result<IpInformation, FailedResponse> {
    let response = match ureq::get("https://funkemunky.cc/vpn")
        .query("ip", ip.as_str())
        .call() {
        Ok(response) => response,
        Err(_) => {
            return Err(FailedResponse {
                success: false,
                reason: "Failed to fetch API request for unknown reason.".to_string(),
            });
        }
    };

    let status = response.status();

    // Checking if the response is HTTP OK (200)
    if status != 200 {
        return Err(FailedResponse {
            success: false,
            reason: format!("Failed to fetch API request with status {status}."),
        });
    }

    let json: Value = match response.into_json() {
        Ok(json) => json,
        Err(_) => {
            return Err(FailedResponse {
                success: false,
                reason: "Failed to parse JSON response.".to_string(),
            });
        }
    };

    let is_success = match json.get("success") {
        Some(success) => success.as_bool().unwrap(),
        None => {
            return Err(FailedResponse {
                success: false,
                reason: "Failed to parse JSON response for status".to_string(),
            });
        }
    };

    if !is_success {
        let failed_response: FailedResponse = serde_json::from_value(json).unwrap();

        return Err(failed_response);
    }

    let ip_info: IpInformation = match serde_json::from_value(json.clone()) {
        Ok(ip_info) => ip_info,
        Err(_) => {
            return Err(FailedResponse {
                success: false,
                reason: format!("Failed to turn successful JSON response into \
                IpInformation object. Raw Response: {}", json.to_string()),
            });
        }
    };
    Ok(ip_info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legitimate_query() {
        let response = match get_ip_info("192.168.1.1".to_string()) {
            Ok(response) => response,
            Err(err) => {
                panic!("Failed to fetch IP information: {}", err.reason);
            }
        };

        if !response.ip.eq("192.168.1.1") {
            panic!("API returned a response with a different IP than we requested.");
        }

        if !response.city.eq("unknown") {
            panic!("API returned a response with a city that is not unknown, \
            which is not expected.");
        }
    }

    #[test]
    fn test_illegitimate_query() {
        let response = match get_ip_info("notanip".to_string()) {
            Ok(response) => {
                panic!("API returned a successful response when it should have failed. Response: {response:?}");
            },
            Err(err) => {
                err
            }
        };

        if response.success {
            panic!("API returned a successful response with a FailedResponse struct when it should have been success=false. Response: {response:?}");
        }
    }

    #[cfg(feature = "async")]
    #[test]
    fn test_async_query() {
        let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();

        let response = rt.block_on(apiasync::alib::get_ip_info_async("192.168.1.1".to_string())).unwrap();

        if !response.ip.eq("192.168.1.1") {
            panic!("API returned a response with a different IP than we requested.");
        }

        if !response.city.eq("unknown") {
            panic!("API returned a response with a city that is not unknown, \
            which is not expected.");
        }
    }
}