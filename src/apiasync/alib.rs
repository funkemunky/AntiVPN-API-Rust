use crate::get_ip_info;
use crate::util::structs::{FailedResponse, IpInformation};

pub async fn get_ip_info_async(ip: String) -> Result<IpInformation, FailedResponse> {
    return get_ip_info(ip);
}