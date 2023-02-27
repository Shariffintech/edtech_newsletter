
#[gitcfg(test)]
mod tests {
    use crate::health_check;

    #[actix_rt::test]
    async fn health_check_works() {
        
        let resp = health_check().await;
        assert!(resp.status().is_success());
    }
}
