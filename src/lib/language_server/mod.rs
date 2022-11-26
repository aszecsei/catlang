use tracing::info;

use lsp_server::{Connection, ExtractError, Message, Request, RequestId, Response};
use lsp_types::{
    request::GotoDefinition, GotoDefinitionResponse, InitializeParams, ServerCapabilities,
};

pub struct LanguageServer {}

impl LanguageServer {
    pub fn run() -> anyhow::Result<()> {
        info!("starting language server...");
        // TODO: will this cause issues with the logger?
        let (connection, io_threads) = Connection::stdio();

        let server_capabilities = serde_json::to_value(&ServerCapabilities::default()).unwrap();
        let initialization_params = connection.initialize(server_capabilities)?;
        Self::main_loop(&connection, initialization_params)?;
        io_threads.join()?;

        info!("shutting down server");
        Ok(())
    }

    fn main_loop(connection: &Connection, params: serde_json::Value) -> anyhow::Result<()> {
        let _params: InitializeParams = serde_json::from_value(params).unwrap();
        for msg in &connection.receiver {
            info!("got msg: {:?}", msg);
            match msg {
                Message::Request(req) => {
                    if connection.handle_shutdown(&req)? {
                        return Ok(());
                    }
                    info!("got request: {:?}", req);
                    match cast::<GotoDefinition>(req) {
                        Ok((id, params)) => {
                            info!("got gotoDefinition request #{}: {:?}", id, params);
                            let result = Some(GotoDefinitionResponse::Array(Vec::new()));
                            let result = serde_json::to_value(&result).unwrap();
                            let resp = Response {
                                id,
                                result: Some(result),
                                error: None,
                            };
                            connection.sender.send(Message::Response(resp))?;
                            continue;
                        }
                        Err(req) => req,
                    };
                }
                Message::Response(resp) => {
                    info!("got response: {:?}", resp);
                }
                Message::Notification(notif) => {
                    info!("got notification: {:?}", notif);
                }
            }
        }
        Ok(())
    }
}

fn cast<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}
