use async_trait::async_trait;
use rust_mcp_sdk::schema::{
    CallToolRequest, CallToolResult, ListToolsRequest, ListToolsResult, RpcError,
    schema_utils::CallToolError,
};
use rust_mcp_sdk::{McpServer, mcp_server::ServerHandler};

use crate::tools::GreetingTools;

// Custom Handler to handle MCP Messages
pub struct MyServerHandler;

// To check out a list of all the methods in the trait that you can override, take a look at
// https://github.com/rust-mcp-stack/rust-mcp-sdk/blob/main/crates/rust-mcp-sdk/src/mcp_handlers/mcp_server_handler.rs

#[async_trait]
#[allow(unused)]
impl ServerHandler for MyServerHandler {
    // Handle ListToolsRequest, return list of available tools as ListToolsResult
    async fn handle_list_tools_request(
        &self,
        request: ListToolsRequest,
        runtime: &dyn McpServer,
    ) -> std::result::Result<ListToolsResult, RpcError> {
        tracing::info!("📥 Received ListToolsRequest: {:?}", request);

        let result = Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools: GreetingTools::tools(),
        });

        tracing::info!("📤 Sending ListToolsResult: {:?}", result);
        result
    }

    /// Handles incoming CallToolRequest and processes it using the appropriate tool.
    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        runtime: &dyn McpServer,
    ) -> std::result::Result<CallToolResult, CallToolError> {
        tracing::info!("📥 Received CallToolRequest: {:?}", request);

        // Attempt to convert request parameters into GreetingTools enum
        let tool_params: GreetingTools =
            GreetingTools::try_from(request.params).map_err(CallToolError::new)?;

        // Match the tool variant and execute its corresponding logic
        let result = match tool_params {
            GreetingTools::SayHelloTool(say_hello_tool) => {
                tracing::info!(
                    "🔧 Executing SayHelloTool with name: {}",
                    say_hello_tool.name
                );
                say_hello_tool.call_tool()
            }
            GreetingTools::SayGoodbyeTool(say_goodbye_tool) => {
                tracing::info!(
                    "🔧 Executing SayGoodbyeTool with name: {}",
                    say_goodbye_tool.name
                );
                say_goodbye_tool.call_tool()
            }
        };

        tracing::info!("📤 Sending CallToolResult: {:?}", result);
        result
    }

    async fn on_server_started(&self, runtime: &dyn McpServer) {
        tracing::info!("🚀 MCP Server started and ready to accept connections");
    }
}
