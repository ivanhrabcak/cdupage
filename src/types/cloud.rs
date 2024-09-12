use serde::{Serialize, Deserialize};
use serde_literals::lit_str;

use super::Serde;

#[derive(Serde!, Serialize)]
#[cfg_attr(
    feature = "node-types",
    derive(TS),
    ts(export),
    ts(rename_all = "camelCase")
)]
pub struct CloudFile {
    #[cfg_attr(not(feature = "node-types"), serde(rename = "cloudid"))]
    pub cloud_id: String,

    pub extension: String,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "type"))]
    pub file_type: String,

    pub file: String,
    pub name: String
}


lit_str!(OkStatus, "ok");

///  If the field is "ok", the value will be `EdupageCloudResponseStatus::Ok`, 
/// if it is any other string, it will be `Other(any_other_string)`
#[derive(Serde!, Serialize)]
#[serde(untagged)]
pub enum EdupageCloudResponseStatus {
    #[serde(with = "OkStatus")]
    Ok,
    Other(String)
}



#[derive(Serde!, Serialize)]
pub struct EdupageCloudResponse {
    pub status: EdupageCloudResponseStatus,

    #[cfg_attr(not(feature = "node-types"), serde(rename = "data"))]
    pub response: Option<CloudFile>
}