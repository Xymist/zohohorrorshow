use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};
use reqwest;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::time::{Duration, SystemTime};
use url::Url;
use webbrowser;

#[derive(Debug, Clone)]
pub struct Credentials {
    client_id: String,
    client_secret: String,
    auth_url: String,
    token_url: String,
    access_token: Option<String>,
    access_token_expiry: Option<SystemTime>,
    refresh_token: Option<String>,
}

impl Credentials {
    pub fn new(
        client_id: &str,
        client_secret: &str,
        auth_url: Option<&str>,
        token_url: Option<&str>,
    ) -> Self {
        Credentials {
            client_id: client_id.to_owned(),
            client_secret: client_secret.to_owned(),
            auth_url: auth_url
                .unwrap_or("https://accounts.zoho.com/oauth/v2/auth")
                .to_owned(),
            token_url: token_url
                .unwrap_or("https://accounts.zoho.com/oauth/v2/token")
                .to_owned(),
            access_token: None,
            access_token_expiry: None,
            refresh_token: None,
        }
    }

    pub fn access_token(&mut self) -> String {
        let missing = self.access_token.is_none();

        if missing || self.outdated() {
            let mut oauth_client = ZohoClient::new(self.clone());

            oauth_client.request_access();
            let creds = oauth_client.credentials();

            self.access_token = creds.raw_access_token();
            self.access_token_expiry = creds.raw_expiry();
        }

        self.access_token.clone().unwrap()
    }

    pub fn raw_access_token(&self) -> Option<String> {
        self.access_token.clone()
    }

    fn raw_expiry(&self) -> Option<SystemTime> {
        self.access_token_expiry
    }

    fn outdated(&self) -> bool {
        match self.access_token_expiry {
            Some(expiry) => expiry.duration_since(SystemTime::now()).is_err(),
            None => true,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct ZohoTokenResponse {
    access_token: Option<String>,
    refresh_token: Option<String>,
    expires_in_sec: usize,
    api_domain: String,
    token_type: String,
    expires_in: usize,
}

struct ZohoClient {
    oauth_client: BasicClient,
    credentials: Credentials,
}

impl ZohoClient {
    pub(in crate::oauth) fn new(credentials: Credentials) -> Self {
        let oauth_client = BasicClient::new(
            ClientId::new(credentials.client_id.clone()),
            Some(ClientSecret::new(credentials.client_secret.clone())),
            AuthUrl::new(credentials.auth_url.clone()).expect("Failed to parse auth URL"),
            Some(TokenUrl::new(credentials.token_url.clone()).expect("Failed to parse token URL")),
        )
        .set_redirect_url(
            RedirectUrl::new("http://localhost:8080/".to_owned()).expect("Invalid redirect URL"),
        );

        ZohoClient {
            oauth_client,
            credentials,
        }
    }

    pub(in crate::oauth) fn request_access(&mut self) {
        let (authorize_url, csrf_state) = self
            .oauth_client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("ZohoProjects.portals.READ".to_string()))
            .add_scope(Scope::new("ZohoProjects.projects.ALL".to_string()))
            .add_scope(Scope::new("ZohoProjects.events.ALL".to_string()))
            .add_scope(Scope::new("ZohoProjects.bugs.ALL".to_string()))
            .add_scope(Scope::new("ZohoProjects.status.READ".to_string()))
            .add_scope(Scope::new("ZohoProjects.status.CREATE".to_string()))
            .add_scope(Scope::new("ZohoProjects.forums.ALL".to_string()))
            .add_scope(Scope::new("ZohoProjects.milestones.ALL".to_string()))
            .add_scope(Scope::new("ZohoProjects.tasks.ALL".to_string()))
            .add_scope(Scope::new("ZohoProjects.tasklists.ALL".to_string()))
            .add_scope(Scope::new("ZohoProjects.users.ALL".to_string()))
            .add_scope(Scope::new("ZohoProjects.documents.READ".to_string()))
            .add_scope(Scope::new("ZohoProjects.documents.CREATE".to_string()))
            .add_scope(Scope::new("ZohoProjects.documents.UPDATE".to_string()))
            .add_scope(Scope::new("ZohoProjects.documents.DELETE".to_string()))
            .add_scope(Scope::new("ZohoPC.files.READ".to_string()))
            .add_scope(Scope::new("ZohoPC.files.CREATE".to_string()))
            .add_scope(Scope::new("ZohoPC.files.DELETE".to_string()))
            // .set_pkce_challenge(pkce_challenge)
            .url();

        if webbrowser::open(&authorize_url.to_string()).is_err() {
            println!(
                "Open this URL in your browser:\n{}\n",
                authorize_url.to_string()
            );
        }

        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                let code;
                let state;
                let now = SystemTime::now();
                {
                    let mut reader = BufReader::new(&stream);

                    let mut request_line = String::new();
                    reader.read_line(&mut request_line).unwrap();

                    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                    let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                    let code_pair = url
                        .query_pairs()
                        .find(|pair| {
                            let &(ref key, _) = pair;
                            key == "code"
                        })
                        .unwrap();

                    let (_, value) = code_pair;
                    code = AuthorizationCode::new(value.into_owned());

                    let state_pair = url
                        .query_pairs()
                        .find(|pair| {
                            let &(ref key, _) = pair;
                            key == "state"
                        })
                        .unwrap();

                    let (_, value) = state_pair;
                    state = CsrfToken::new(value.into_owned());
                }

                let message = "Authenticated successfully. You can now close this tab.";
                let response = format!(
                    "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                    message.len(),
                    message
                );
                stream.write_all(response.as_bytes()).unwrap();

                if state.secret() != csrf_state.secret() {
                    println!(
                        "CSRF Tokens appear to differ. Please check:\n{:?}\n{:?}",
                        state.secret(),
                        csrf_state.secret()
                    )
                }

                // Exchange the code with a token.
                let token_response = self.exchange_code(code.clone());

                self.credentials.access_token = token_response.access_token;
                self.credentials.access_token_expiry =
                    Some(now + Duration::from_secs(token_response.expires_in_sec as u64));
                self.credentials.refresh_token = token_response.refresh_token;

                // The server will terminate itself after collecting the first code.
                break;
            }
        }
    }

    fn exchange_code(&self, code: AuthorizationCode) -> ZohoTokenResponse {
        let req_client: reqwest::Client = reqwest::Client::new();
        let mut builder =
            req_client.request(reqwest::Method::POST, &self.credentials.token_url.clone());
        let mut params = HashMap::new();
        params.insert("code".to_owned(), code.secret().to_owned());
        params.insert(
            "redirect_uri".to_owned(),
            "http://localhost:8080/".to_owned(),
        );
        params.insert("client_id".to_owned(), self.credentials.client_id.clone());
        params.insert(
            "client_secret".to_owned(),
            self.credentials.client_secret.clone(),
        );
        params.insert("grant_type".to_owned(), "authorization_code".to_owned());

        builder = builder.query(&params);

        // TODO(Xymist): Error handling
        let token_response: ZohoTokenResponse = builder.send().unwrap().json().unwrap();
        token_response
    }

    pub(in crate::oauth) fn credentials(&self) -> Credentials {
        self.credentials.clone()
    }
}
