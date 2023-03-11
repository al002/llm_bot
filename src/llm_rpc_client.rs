use tonic::transport::Channel;
use crate::llm::llm_client::LlmClient;

pub struct LLMRpcClient {
    pub client: LlmClient<Channel>,
}

impl LLMRpcClient {
    pub async fn new(dst: String) -> Self {
        let client = LlmClient::connect(dst).await.unwrap();

        LLMRpcClient {
            client,
        }
    } 
}

