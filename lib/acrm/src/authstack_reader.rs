/*
Examples of Redis Session token key and object structure:
---------------------------------------------------------
Redis key:
    /AppSession/20240411175445c44224ff853098adf0ef3f376a7ffbb6ddad2213803c47743f/AuthStack
Redis object:
    Public session:
    {
        "AuthTime": 1713896812.4246874,
        "RefreshTime": 1713896818.4582505,
        "Stack": [
            {
                "Key": ["Public"],
                "PreviousURI": null,
                "AdminMode": false,
                "WriteMode": false,
                "SuperMode": false,
                "DebugMode": false,
                "LockDownType": null,
                "AdminPerm": false,
                "SuperPerm": false,
                "DebugPerm": false,
                "Name": "Public",
            }
        ]
    }

    Private session:
    {
        "AuthTime": 1713296017.1831234,
        "RefreshTime": 1713301827.9128642,a
        "Stack": [
            {
                "Key": ["ACRM_Contact", 110076, null],
                "PreviousURI": null,
                "AdminMode": true,
                "WriteMode": false,
                "SuperMode": false,
                "DebugMode": false,
                "LockDownType": null,
                "Contact_GSID": "JEVeJj95To9T0Kvo",
                "Contact_ZNID": 10092175,
                "Name": "James Garber",
                "Email": "foo@appcove.com",
                "AdminPerm": true,
                "SuperPerm": false,
                "DebugPerm": false,
                "TagCache": [1, 3109, 10015158, 10917829],
                "TimeZone": "US/Eastern",
                "Account_GSID": null,
                "Account_Name": null,
                "Contact_AccountManagementPerm": null,
                "Account_List": [],
                "Account_ZNID": null,
                "ProfilePictureURI": "https://smartadvisortools-com.local.acp7.net/AppStruct/DefaultThumbnail.png"
            }
        ]
    }
*/

use granite_redis::RedisCX;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Contact_ZNID")]
pub struct ContactZNID(u32);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "Account_ZNID")]
pub struct AccountZNID(u32);

#[derive(Debug, Serialize, Deserialize)]
pub enum KeyType {
    Public(String),
    #[serde(rename = "ACRM_Contact")]
    ACRMContact(String),
}

// TODO: Figure out the proper way to deserialize the Key tuples
#[derive(Debug, Serialize, Deserialize)]
pub enum KeyData {
    Public((KeyType,)),
    #[serde(rename = "ACRM_Contact")]
    ACRMContact((KeyType, ContactZNID, Option<AccountZNID>)),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "Key")]
    pub key: serde_json::Value,
    #[serde(rename = "PreviousURI")]
    pub previous_uri: Option<String>,
    #[serde(rename = "AdminPerm")]
    pub admin_perm: bool,
    #[serde(rename = "AdminMode")]
    pub admin_mode: bool,
    #[serde(rename = "WriteMode")]
    pub write_mode: bool,
    #[serde(rename = "SuperPerm")]
    pub super_perm: bool,
    #[serde(rename = "SuperMode")]
    pub super_mode: bool,
    #[serde(rename = "DebugPerm")]
    pub debug_perm: bool,
    #[serde(rename = "DebugMode")]
    pub debug_mode: bool,
    #[serde(rename = "LockDownType")]
    pub lock_down_type: Option<String>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Email")]
    pub email: Option<String>,
    #[serde(rename = "TagCache")]
    pub tag_cache: Option<Vec<u32>>,
    #[serde(rename = "TimeZone")]
    pub time_zone: Option<String>,
    #[serde(rename = "ProfilePictureURI")]
    pub profile_picture_uri: Option<String>,
    #[serde(rename = "Contact_ZNID")]
    pub contact_znid: Option<u32>,
    #[serde(rename = "Contact_GSID")]
    pub contact_gsid: Option<String>,
    #[serde(rename = "Account_ZNID")]
    pub account_znid: Option<u32>,
    #[serde(rename = "Account_GSID")]
    pub account_gsid: Option<String>,
    #[serde(rename = "Account_Name")]
    pub account_name: Option<String>,
    #[serde(rename = "Contact_AccountManagementPerm")]
    pub account_management_perm: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthStack {
    #[serde(rename = "AuthTime")]
    pub auth_time: f64,
    #[serde(rename = "RefreshTime")]
    pub refresh_time: f64,
    #[serde(rename = "Stack")]
    pub stack: Vec<User>,
}

impl AuthStack {
    pub fn get_current_user(&self) -> Option<&User> {
        let user = self.stack.last();

        match user {
            Some(user) => Some(user),
            None => None,
        }
    }

    pub fn has_admin_perm(&self) -> bool {
        for user in self.stack.iter().rev() {
            if user.admin_perm {
                return true;
            }
        }

        false
    }
}

fn get_session_key(token: &String) -> String {
    format!("/AppSession/{}/AuthStack", token)
}

pub async fn get_auth_info(
    redis: &mut RedisCX<'_>,
    session_token: &String,
) -> granite::Result<AuthStack> {
    let session_key = get_session_key(session_token);

    let auth_info = match redis.get_json::<AuthStack>(&session_key).await {
        Ok(auth_info) => auth_info,
        Err(_) => AuthStack {
            auth_time: 0.0,
            refresh_time: 0.0,
            stack: vec![],
        },
    };

    Ok(auth_info)
}
