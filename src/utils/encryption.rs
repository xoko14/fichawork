use crate::errors::ApiError;

pub async fn hash(str: String) -> Result<String, ApiError> {
    let (rx, tx) = tokio::sync::oneshot::channel();

    rayon::spawn(move || {
        let hash = bcrypt::hash(str, bcrypt::DEFAULT_COST);
        _ = rx.send(hash);
    });

    let hash = tx.await??;
    Ok(hash)
}

pub async fn verify_hash(str: String, hash: String) -> Result<bool, ApiError> {
    let (rx, tx) = tokio::sync::oneshot::channel();

    rayon::spawn(move || {
        let result = bcrypt::verify(str, &hash);
        _ = rx.send(result);
    });

    let result = tx.await??;
    Ok(result)
}
