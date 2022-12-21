use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VaultRepoCreate {
    #[serde(rename = "mountId")]
    pub mount_id: String,
    pub path: String,
    pub salt: Option<String>,
    #[serde(rename = "passwordValidator")]
    pub password_validator: String,
    #[serde(rename = "passwordValidatorEncrypted")]
    pub password_validator_encrypted: String,
}
