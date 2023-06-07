// Nightly clippy (0.1.64) considers Drop a side effect, see https://github.com/rust-lang/rust-clippy/issues/9608
#![allow(clippy::unnecessary_lazy_evaluations)]

use anyhow::Result;
use k8s_openapi::api::core::v1::ConfigMap;
use kube::{
    api::{Api, ObjectMeta, PostParams},
    Client,
};
use kube_core::Resource;
use std::collections::BTreeMap;

#[tokio::main]
async fn main() -> Result<()> {
    let pp = PostParams::default();
    let client = Client::try_default().await?;
    let cm_api = Api::<ConfigMap>::namespaced(client.clone(), "default");
    let cm_name = "my-configmap".to_string();
    let cm = ConfigMap {
        metadata: ObjectMeta {
            name: Some(cm_name.clone()),
            ..ObjectMeta::default()
        },
        data: None,
        ..Default::default()
    };

    cm_api.create(&pp, &cm).await?;

    let created_cm = cm_api.get(&cm_name).await.unwrap();
    println!(
        "rv is {} before update",
        created_cm.metadata.resource_version.as_ref().unwrap()
    );

    cm_api.replace(&cm_name, &pp, &created_cm).await?;

    let updated_cm = cm_api.get(&cm_name).await.unwrap();
    println!(
        "rv is {} after update 1",
        updated_cm.metadata.resource_version.unwrap()
    );

    let updated_cm2 = cm_api.get(&cm_name).await.unwrap();
    println!(
        "rv is {} after update 2",
        updated_cm2.metadata.resource_version.unwrap()
    );
    // The two rv are the same

    Ok(())
}
