// TODO: move this to a separate crate
use crate::constants::MatrixWebApi;
use crate::settings::config::ConfigConstants;
use crate::settings::ConstArg;
use clap::{App, Arg};
use matrix_web_security::Secret;

impl From<ConstArg> for Arg<'static, 'static> {
    fn from(src: ConstArg) -> Self {
        let arg = Arg::with_name(src.name)
            .long(src.long)
            .required(src.required);

        if src.short.is_some() && src.value.is_some() {
            arg.short(src.get_short())
                .default_value(src.get_value())
                .takes_value(true)
        } else if src.short.is_some() {
            arg.short(src.get_short())
        } else if src.value.is_some() {
            arg.default_value(src.get_value()).takes_value(true)
        } else {
            arg
        }
    }
}

pub struct CommandLine;

impl CommandLine {
    pub fn new<'a, 'b>(secret: &'a Secret) -> App<'a, 'b> {
        App::new(MatrixWebApi::APP_NAME)
            .version(MatrixWebApi::APP_VERSION)
            .author(MatrixWebApi::APP_AUTHOR)
            .arg(Arg::from(ConfigConstants::REDIRECT_URI))
            .arg(Arg::from(ConfigConstants::SYNAPSE_URI))
            .arg(Arg::from(ConfigConstants::IP))
            .arg(Arg::from(ConfigConstants::PORT))
            .arg(Arg::from(ConfigConstants::BASE_URI))
            .arg(Arg::from(ConfigConstants::STATIC_PATH))
            .arg(Arg::from(ConfigConstants::HOME_SERVER))
            .arg(Arg::from(ConfigConstants::AUTHORITY))
            .arg(Arg::from(ConfigConstants::CLIENT_API))
            .arg(Arg::from(ConfigConstants::SECRET_KEY))
            .arg(Arg::from(ConfigConstants::SECRET).default_value(secret.as_str()))
    }
}
