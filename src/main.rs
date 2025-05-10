use rmcp::{
    Error, ServerHandler, ServiceExt,
    model::{CallToolResult, Content, Implementation, ServerCapabilities, ServerInfo},
    tool,
    transport::stdio,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = TextAnalyzer::new().serve(stdio()).await.inspect_err(|e| {
        println!("{e}");
    })?;
    service.waiting().await?;
    Ok(())
}

#[derive(Clone, Debug)]
pub struct TextAnalyzer {}

#[tool(tool_box)]
impl TextAnalyzer {
    pub fn new() -> Self {
        Self {}
    }

    #[tool(description = "Count the number of characters")]
    pub async fn count_characters(
        &self,
        #[tool(param)] s: String,
    ) -> Result<CallToolResult, Error> {
        Ok(CallToolResult::success(vec![Content::text(
            s.chars().count().to_string(),
        )]))
    }
}

#[tool(tool_box)]
impl ServerHandler for TextAnalyzer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(String::from("The server provides text manipulation tools.")),
            ..Default::default()
        }
    }
}
