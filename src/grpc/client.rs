use crate::proto;

/// Returns a
/// [`crate::proto::StockRequest`](../../proto/stock/struct.StockRequest.html)
/// given a [`crate::proto::Store`](../../proto/stock/enum.Store.html).
/// Also, it receives a boolean parameter `show_unavailable` that will
/// decide if unavailable items must be returned.
pub fn generate_list_request(store: proto::Store, show_unavailable: bool) -> proto::StockRequest {
    let mut req = proto::StockRequest::new();
    req.set_store(store);
    req.set_display_unavailable_items(show_unavailable);
    req
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_list_request() {
        assert_eq!(
            generate_list_request(proto::Store::BERLIN_DE, true).get_store(),
            proto::Store::BERLIN_DE
        );
        assert_eq!(
            generate_list_request(proto::Store::VENEZA_IT, true).get_store(),
            proto::Store::VENEZA_IT
        );
        assert_eq!(
            generate_list_request(proto::Store::BERLIN_DE, true).get_display_unavailable_items(),
            true
        );
        assert_eq!(
            generate_list_request(proto::Store::VENEZA_IT, false).get_display_unavailable_items(),
            false
        );
    }
}
