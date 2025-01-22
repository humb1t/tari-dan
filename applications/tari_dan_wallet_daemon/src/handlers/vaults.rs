use tari_dan_wallet_sdk::apis::jwt::JrpcPermission;
use tari_wallet_daemon_client::types::{VaultGetRequest, VaultGetResponse};

use crate::handlers::HandlerContext;

const _LOG_TARGET: &str = "tari::dan::wallet_daemon::json_rpc::vaults";

pub async fn handle_vaults_get(
    context: &HandlerContext,
    token: Option<String>,
    req: VaultGetRequest,
) -> Result<VaultGetResponse, anyhow::Error> {
    let sdk = context.wallet_sdk();
    sdk.jwt_api().check_auth(token, &[JrpcPermission::Admin])?;
    let vault = context.wallet_sdk().accounts_api().get_vault(&req.vault_id.into())?;
    Ok(VaultGetResponse { vault })
}
