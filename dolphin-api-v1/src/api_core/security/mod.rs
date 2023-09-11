use self::authenticator::{
    Authenticator,
    AuthenticatorType,
    LdapAuthenticator,
    PasswordAuthenticator,
};
use dolphin_config::api_config::Settings;
pub mod authenticator;

trait AuthenticatorFactory {
    fn build() -> Box<dyn Authenticator>;
}

struct PwdAuthenticatorFactory;
struct LdapAuthenticatorFactory;


impl AuthenticatorFactory for LdapAuthenticatorFactory {
    fn build() -> Box<dyn Authenticator> {
        Box::<LdapAuthenticator>::default()
    }
}
impl AuthenticatorFactory for PwdAuthenticatorFactory {
    fn build() -> Box<dyn Authenticator> {
        Box::<PasswordAuthenticator>::default()
    }
}


pub fn get_authenticator() -> Box<dyn Authenticator> {
    let settings = Settings::new().unwrap();
    let auth_type = settings
        .security
        .authentication_type
        .unwrap_or("PASSWORD".to_string());
    let r#type = AuthenticatorType::new(auth_type);
    match r#type {
        AuthenticatorType::Password => PwdAuthenticatorFactory::build(),
        AuthenticatorType::Ldap => LdapAuthenticatorFactory::build(),
    }
}
