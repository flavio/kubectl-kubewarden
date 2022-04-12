use anyhow::{anyhow, Result};
use std::fs;

wit_bindgen_rust::import!("wit/ephemeral/wasi-outbound-http.wit");

pub struct ConnectionConfig {
    pub identity: UserIdentity,
    pub server: Server,
}

impl ConnectionConfig {
    pub fn from_kube_config() -> Result<ConnectionConfig> {
        let config = kube_conf::Config::load_default()
            .map_err(|e| anyhow!("kubeconf: cannot read config: {:?}", e))?;

        let kube_ctx = config
            .get_current_context()
            .ok_or(anyhow!("kubeconf: no default kubernetes context"))?;

        let cluster = kube_ctx
            .get_cluster(&config)
            .ok_or(anyhow!("kubeconf: cannot find cluster definition"))?;

        let user = kube_ctx
            .get_user(&config)
            .ok_or(anyhow!("kubeconf: cannot find user definition"))?;

        let identity = UserIdentity::from_kube_user_and_cluster(&user, &cluster)?;
        let server = Server::from_cluster(&cluster)?;

        Ok(ConnectionConfig { identity, server })
    }

    pub fn register<'a>(&'a self) -> Result<String> {
        let accept_invalid_hostnames = false;
        let accept_invalid_certificates = false;

        let server_cert = wasi_outbound_http::Certificate {
            encoding: wasi_outbound_http::CertificateEncoding::Pem,
            data: &self.server.ca,
        };
        let extra_root_certificates = vec![server_cert];

        let ui: &'a UserIdentity = &self.identity;
        let identity: wasi_outbound_http::Identity<'a> = ui.into();

        let req_cfg = wasi_outbound_http::RequestConfig {
            accept_invalid_hostnames,
            accept_invalid_certificates,
            extra_root_certificates: extra_root_certificates.as_slice(),
            identity: Some(identity),
        };

        wasi_outbound_http::register_request_config(req_cfg, None)
            .map_err(|e| anyhow!("register request config error: {:?}", e))
    }
}

pub struct UserIdentity {
    pub key: Vec<u8>,
    pub cert: Vec<u8>,
    pub ca: Vec<u8>,
}

impl UserIdentity {
    fn from_kube_user_and_cluster(
        user: &kube_conf::user::User,
        cluster: &kube_conf::cluster::Cluster,
    ) -> Result<Self> {
        let ca = if let Some(data) = cluster.certificate_authority_data.as_ref() {
            base64::decode(data).map_err(|e| {
                anyhow!(
                    "kubeconf: cannot decode embedded server certificate - {}",
                    e
                )
            })
        } else if let Some(path) = cluster.certificate_authority.as_ref() {
            fs::read(path).map_err(|e| anyhow!("kubeconf: cannot read server certificate - {}", e))
        } else {
            Err(anyhow!("kubeconf: cannot determine cluster CA"))
        }?;

        let key = if let Some(data) = user.client_key_data.as_ref() {
            base64::decode(data)
                .map_err(|e| anyhow!("kubeconf: cannot decode embedded user key - {}", e))
        } else if let Some(path) = user.client_key.as_ref() {
            fs::read(path).map_err(|e| anyhow!("kubeconf: cannot read client key - {}", e))
        } else {
            Err(anyhow!("kubeconf: cannot determine user key"))
        }?;

        let cert = if let Some(data) = user.client_certificate_data.as_ref() {
            base64::decode(data)
                .map_err(|e| anyhow!("kubeconf: cannot decode embedded user cert - {}", e))
        } else if let Some(path) = user.client_certificate.as_ref() {
            fs::read(path).map_err(|e| anyhow!("kubeconf: cannot read client cert - {}", e))
        } else {
            Err(anyhow!("kubeconf: cannot determine user certificate"))
        }?;

        Ok(UserIdentity { key, cert, ca })
    }
}

pub struct Server {
    pub url: String,
    pub ca: Vec<u8>,
}

impl Server {
    fn from_cluster(cluster: &kube_conf::cluster::Cluster) -> Result<Self> {
        let ca = if let Some(data) = cluster.certificate_authority_data.as_ref() {
            base64::decode(data).map_err(|e| {
                anyhow!(
                    "kubeconf: cannot decode embedded server certificate - {}",
                    e
                )
            })
        } else if let Some(path) = cluster.certificate_authority.as_ref() {
            fs::read(path).map_err(|e| anyhow!("kubeconf: cannot read server certificate - {}", e))
        } else {
            Err(anyhow!("kubeconf: cannot determine cluster CA"))
        }?;

        Ok(Server {
            ca,
            url: cluster.server.clone(),
        })
    }
}

impl<'a> From<&'a UserIdentity> for wasi_outbound_http::Identity<'a> {
    fn from(ui: &'a UserIdentity) -> Self {
        wasi_outbound_http::Identity {
            key: &ui.key,
            cert: &ui.cert,
            ca: &ui.ca,
        }
    }
}

//impl<'a> From<&'a ConnectionConfig> for wasi_outbound_http::RequestConfig<'a> {
//    fn from(cfg: &'a ConnectionConfig) -> Self {
//        let accept_invalid_hostnames = false;
//        let accept_invalid_certificates = false;

//        let server_cert_data: &'a [u8] = &cfg.server.ca;
//        let server_cert = wasi_outbound_http::Certificate {
//            encoding: wasi_outbound_http::CertificateEncoding::Pem,
//            data: server_cert_data,
//        };
//        let extra_root_certificates = vec![server_cert];

//        let ui: &'a UserIdentity = &cfg.identity;
//        let identity: wasi_outbound_http::Identity<'a> = ui.into();

//        wasi_outbound_http::RequestConfig {
//            accept_invalid_hostnames,
//            accept_invalid_certificates,
//            extra_root_certificates: extra_root_certificates.as_slice(),
//            identity: Some(identity),
//        }
//    }
//}
