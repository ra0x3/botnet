/// Botnet middleware middleware utils.
pub mod middleware;

/// Botnet core.
pub mod core {

    /// Re-exported `botnet_core`.
    pub use botnet_core::*;
}

/// Botnet prelude.
pub mod prelude {

    /// Re-exported `crate::core::prelude`.
    pub use crate::core::prelude::*;

    /// Re-exported `botnet_macros`.
    pub use botnet_macros::*;

    /// Re-exported `botnet::middleware`.
    pub use crate::middleware::*;

    /// Re-exported `botnet::*`.
    pub use crate::botnet;
}

/// Botnet macros.
pub mod macros {

    /// Re-exported `botnet_macros`.
    pub use botnet_macros::*;
}

/// Botnet utils.
pub mod utils {

    /// Re-exported `botnet_utils`.
    pub use botnet_utils::*;
}

use async_std::sync::Arc;

/// Re-export of `#[tokio::main]`.
pub use botnet_macros::main;

/// Run botnet.
///
/// This function is the entry point for botnet operations that can't
/// be plugged into `botnet::middleware`.
///
/// This is most likely how botnet will most often be called from arbitrary contexts.
pub async fn botnet<E, A, C>(
    context: Arc<crate::core::context::BotnetContext<E, A, C>>,
    input: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>>
where
    E: crate::core::config::EntityCounter,
    A: crate::core::config::Anonimity + Default,
    C: crate::core::config::RateLimit,
{
    let input = crate::core::models::input::Input::from(input);
    let _keys = context
        .keys()
        .iter()
        .map(|(ty_id, _)| {
            let exts = context
                .get_extractors(ty_id)
                .expect("Invalid type ID for extractors.");
            let meta = context
                .get_metadata(ty_id)
                .expect("Invalid type ID for metadata.");
            crate::core::models::key::BotnetKey::from((&input, exts, meta))
        })
        .collect::<Vec<crate::core::models::key::BotnetKey>>();

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{core::Url, prelude::*};
    use std::path::PathBuf;

    #[botest(config = "config.yaml")]
    fn test_botnet() {
        let key_id = 3472328297305896040;
        let config_key = context.get_key(&key_id).unwrap();

        assert_eq!(config_key.name, "http".to_string());
        assert_eq!(config_key.fields.len(), 2);

        let field = config_key.fields.first().unwrap();
        assert_eq!(field.key, "ssl");
        assert_eq!(field.name, "ssl");
        assert_eq!(field.description, Some("SSL parameter.".to_string()));

        let input: input::Input =
            Url::parse("http://localhost:8080/api/v1/foo/bar?zoo=1&baz=true&region=usw&ssl=v1.3&ip=1.1.1.1")
                .unwrap()
                .into();

        let extractors = context.get_extractors(&key_id).unwrap();
        let metadata = context.get_metadata(&key_id).unwrap();
        let extracted_key = key::BotnetKey::from((&input, extractors, metadata));

        assert_eq!(extracted_key.name(), config_key.name);
        assert_eq!(extracted_key.fields().len(), config_key.fields.len());
    }
}
