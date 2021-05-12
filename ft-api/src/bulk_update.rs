#[derive(Serialize)]
struct BulkUpdateInput {
    collection: String,
    auth_code: String,
    current_hash: String,
    new_hash: String,
    repo: String,
    files: Vec<Action>,
}

#[derive(Serialize)]
struct File {
    id: String,
    content: String,
}

pub fn bulk_update(
    collection: &str,
    current_hash: &str,
    new_hash: &str,
    repo: &str,
    files: Vec<Action>,
    auth_code: &str,
) -> crate::Result<()> {
    let url = format!("/{}/~/bulk-update/", collection);

    let update = BulkUpdateInput {
        collection: collection.trim().to_string(),
        auth_code: auth_code.trim().to_string(),
        current_hash: current_hash.trim().to_string(),
        new_hash: new_hash.trim().to_string(),
        repo: repo.trim().to_string(),
        files,
    };

    #[derive(Serialize)]
    struct UpdatedWrapper {
        data: BulkUpdateInput,
    }

    crate::api::action::<crate::sync_status::Status, _>(
        &url,
        UpdatedWrapper { data: update },
        None,
    )?;
    Ok(())
}

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum Action {
    Updated { id: String, content: String },
    Added { id: String, content: String },
    Deleted { id: String },
}
