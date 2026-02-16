use std::future::Future;

use crate::error::ApiClientError;

/// Generic pagination helper that collects all pages into a single Vec.
///
/// - `fetch_page`: async function that takes an optional page token and returns (items, next_page_token)
///
/// Calls `fetch_page(None)` for the first page, then `fetch_page(Some(token))` for subsequent
/// pages until the next_page_token is `None` or empty.
pub async fn paginate<T, F, Fut>(fetch_page: F) -> Result<Vec<T>, ApiClientError>
where
    F: Fn(Option<String>) -> Fut,
    Fut: Future<Output = Result<(Vec<T>, Option<String>), ApiClientError>>,
{
    let mut all_items = Vec::new();
    let mut page_token: Option<String> = None;

    loop {
        let (items, next_token) = fetch_page(page_token).await?;
        all_items.extend(items);

        match next_token {
            Some(token) if !token.is_empty() => {
                page_token = Some(token);
            }
            _ => break,
        }
    }

    Ok(all_items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn paginate_single_page() {
        let result = paginate(|_token| async { Ok((vec![1, 2, 3], None)) }).await;
        assert_eq!(result.unwrap(), vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn paginate_multiple_pages() {
        let call_count = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
        let cc = call_count.clone();

        let result = paginate(move |token| {
            let cc = cc.clone();
            async move {
                let count = cc.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                match count {
                    0 => {
                        assert!(token.is_none());
                        Ok((vec![1, 2], Some("page2".to_string())))
                    }
                    1 => {
                        assert_eq!(token.as_deref(), Some("page2"));
                        Ok((vec![3, 4], Some("page3".to_string())))
                    }
                    2 => {
                        assert_eq!(token.as_deref(), Some("page3"));
                        Ok((vec![5], None))
                    }
                    _ => panic!("too many calls"),
                }
            }
        })
        .await;

        assert_eq!(result.unwrap(), vec![1, 2, 3, 4, 5]);
        assert_eq!(call_count.load(std::sync::atomic::Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn paginate_empty_token_stops() {
        let result = paginate(|_token| async { Ok((vec![1], Some(String::new()))) }).await;
        assert_eq!(result.unwrap(), vec![1]);
    }

    #[tokio::test]
    async fn paginate_error_propagates() {
        let result: Result<Vec<i32>, _> = paginate(|_token| async {
            Err(ApiClientError::Api {
                status: 500,
                body: "server error".to_string(),
            })
        })
        .await;
        assert!(result.is_err());
    }
}
