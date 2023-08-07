use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{LitStr, Token};

mod kw {
    syn::custom_keyword!(config);
}

pub(crate) struct BotnetMainArgs {
    pub(crate) path: String,
}

struct ConfigBuilder {
    path: Option<Config>,
}

impl ConfigBuilder {
    fn new() -> ConfigBuilder {
        ConfigBuilder { path: None }
    }

    fn set_config(&mut self, path: Config) {
        self.path = Some(path)
    }

    fn build(self) -> BotnetMainArgs {
        let ConfigBuilder { path } = self;

        BotnetMainArgs {
            path: path
                .expect("`path` specification is required in indexer definition.")
                .name
                .value(),
        }
    }
}

impl Parse for BotnetMainArgs {
    fn parse(input: ParseStream) -> syn::Result<BotnetMainArgs> {
        let mut path = ConfigBuilder::new();

        let items = Punctuated::<ConfigItem, Token![,]>::parse_terminated(input)?;

        for item in items {
            match item {
                ConfigItem::Config(s) => path.set_config(s),
            }
        }

        Ok(path.build())
    }
}

enum ConfigItem {
    Config(Config),
}

impl Parse for ConfigItem {
    fn parse(input: ParseStream) -> syn::Result<ConfigItem> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::config) {
            Ok(input.parse().map(ConfigItem::Config)?)
        } else {
            Err(lookahead.error())
        }
    }
}

struct Config {
    name: LitStr,
}

impl Parse for Config {
    fn parse(input: ParseStream) -> syn::Result<Config> {
        let _: kw::config = input.parse()?;
        let _: Token![=] = input.parse()?;
        let name = input.parse()?;

        Ok(Config { name })
    }
}
