mod common;

#[cfg(test)]
mod dspace_version_api_test {

    use crate::common::setup_consumer_configuration;

    #[tokio::test]
    async fn test_get_dspace_version() {
        let configuration = setup_consumer_configuration();
        let result = dsp_client::common::dspace_version::get_dspace_version(&configuration).await;

        assert!(result.is_ok());
    }
}
