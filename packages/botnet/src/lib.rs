use botnet_core::{
    models::{BotnetKey, Input},
    task::Strategy,
};
use botnet_utils::type_id;

#[macro_export]
macro_rules! botnet_ctx {
    ($mod: pat) => {

        use #r#mod;

    };
}

/// Botnet middleware middleware utils.
pub mod middleware;

/// Botnet core.
pub mod core {

    /// Re-exported `botnet_core`.
    pub use botnet_core::*;
}

/// Botnet prelude.
pub mod prelude {

    /// Re-exported `botnet_core::prelude`.
    pub use botnet_core::prelude::*;

    /// Re-exported `botnet_macros`.
    pub use botnet_macros::*;

    /// Re-exported `botnet::middleware`.
    pub use crate::middleware::*;

    /// Re-exported `botnet::*`.
    pub use crate::{botnet, botnet_ctx};
}

/// Botnet macros.
pub mod macros {

    /// Re-exported `botnet_macros`.
    pub use botnet_macros::*;

    pub use crate::botnet_ctx;
}

/// Botnet utils.
pub mod utils {

    /// Re-exported `botnet_utils`.
    pub use botnet_utils::*;
}

use std::rc::Rc;

/// Re-export of `#[tokio::main]`.
pub use botnet_macros::main;

/// Run botnet.
///
/// This function is the entry point for botnet operations that can't
/// be plugged into `botnet::middleware`.
///
/// This is most likely how botnet will most often be called from arbitrary contexts.
pub async fn botnet(
    context: Rc<botnet_core::context::BotnetContext>,
    input: Vec<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    let input = Input::from(input);

    let extractors = context.extractors();
    let metadata = context.metadata();

    let _keys = context
        .keys()
        .iter()
        .map(|(_, k)| {
            let ty_id = type_id(k.name());
            let exts = extractors.get(&ty_id).unwrap();
            let meta = metadata.get(&ty_id).unwrap();

            BotnetKey::from_input(&input, exts, meta).unwrap_or_default()
        })
        .collect::<Vec<BotnetKey>>();

    let _strategy = Strategy::new(context.clone());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use botnet_core::{config::BotnetConfig, context::BotnetContext, Url};
    use std::path::PathBuf;

    #[test]
    fn test_can_run_botnet_from_config() {
        // Check config.
        let config = BotnetConfig::from(PathBuf::from("./../../config.yaml.example"));
        assert_eq!(config.keys.len(), 2);

        let _input: Input =
            Url::parse("http://localhost:8080/api/v1/foo/bar?zoo=1&baz=true&region=usw")
                .unwrap()
                .into();

        let context = BotnetContext::default();

        assert_eq!(context.keys().len(), 2);
    }
}
