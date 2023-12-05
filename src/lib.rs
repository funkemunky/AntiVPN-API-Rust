mod util;

use util::structs::{IpInformation, FailedResponse};
use serde_json::Value;

pub fn get_ip_info(ip: String) -> Result<IpInformation, FailedResponse> {


    let response = match ureq::get("https://funkemunky.cc/vpn").query("ip", ip.as_str()).call() {
        Ok(response) => response,
        Err(_) => {
            return Err(FailedResponse {
                success: false,
                reason: "Failed to fetch API request for unknown reason.".to_string()
            });
        }
    };

    let status = response.status();

    // Checking if the response is HTTP OK (200)
    if status != 200 {
        return Err(FailedResponse {
            success: false,
            reason: format!("Failed to fetch API request with status {status}.")
        });
    }

    let raw_string_response = match response.into_string() {
        Ok(json) => json,
        Err(_) => {
            return Err(FailedResponse {
                success: false,
                reason: "Failed to parse JSON response.".to_string()
            });
        }
    };

    println!("Raw string response: {raw_string_response}");

    let json: Value = match serde_json::from_str(raw_string_response.as_str()) {
        Ok(json) => json,
        Err(_) => {
            return Err(FailedResponse {
                success: false,
                reason: "Failed to parse JSON response.".to_string()
            });
        }
    };

    let is_success = match json.get("success") {
        Some(success) => success.as_bool().unwrap(),
        None => {
            return Err(FailedResponse {
                success: false,
                reason: "Failed to parse JSON response for status".to_string()
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
                reason: format!("Failed to turn successful JSON response into IpInformation object. Raw Response: {}", json.to_string())
            });
        }
    };
    Ok(ip_info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let response = match get_ip_info("192.168.1.1".to_string()) {
            Ok(response) => response,
            Err(err) => {
                panic!("Failed to fetch IP information: {}", err.reason);
            }
        };

        if response.ip.eq("192.168.1.1") {
           println!("API returned a response with the same IP as we requested");
        } else {
            panic!("API returned a response with a different IP than we requested.");
        }

        if response.city.eq("unknown") {
            println!("API returned a response with the city as unknown, which is expected.");
        } else {
            panic!("API returned a response with a city that is not unknown, which is not expected.");
        }
    }
}
