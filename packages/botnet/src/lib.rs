/// Botnet middleware service utils.
pub mod service;

/// Botnet core.
pub mod core {

    /// Re-exported `botnet_core`.
    pub use botnet_core::*;
}

/// Botnet prelude.
pub mod prelude {

    /// Re-exported `botnet_core::prelude`.
    pub use botnet_core::prelude::*;
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
