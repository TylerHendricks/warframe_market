use crate::{
    BASE_URL, CurrentUser, CurrentUserContainer, MarketClient, MarketResponseV1, V1_URL,
    market_error::MarketError,
};
use std::collections::HashMap;

impl MarketClient {
    pub async fn sign_in(
        &mut self,
        email: &str,
        password: &str,
        device_id: &str,
    ) -> Result<CurrentUser, MarketError> {
        let mut body = HashMap::new();
        body.insert("auth_type", "header");
        body.insert("email", email);
        body.insert("password", password);
        body.insert("device-id", device_id);

        let mut request = self
            .reqwest_client
            .post(format!("{V1_URL}auth/signin"))
            .json(&body);

        // This header needs to be present
        request = request.header("Authorization", "");

        let response = request.send().await?;
        if response.status() == 429 {
            return Err(MarketError::TooManyRequests);
        }

        // Successful sign in response includes an authorization header and token
        if let Some(token) = response.headers().get("authorization") {
            // Produces a value like: "JWT <token_string>"
            let mut token_str = token.to_str()?;
            // Removes the leading "JWT ", leaving only the token string
            token_str = &token_str[4..];

            self.auth_token = Some(token_str.to_string());

            let json = response.text().await?;
            let market_response: MarketResponseV1<CurrentUserContainer> =
                serde_json::from_str(&json)?;
            let current_user = market_response.payload.user;
            Ok(current_user)
        } else {
            Err(MarketError::SignInFailed)
        }
    }

    /// Terminate current session.
    /// Refresh and access tokens will become unusable.
    pub async fn sign_out(&mut self) -> Result<(), MarketError> {
        let request = self.reqwest_client.post(format!("{BASE_URL}auth/signout"));
        request.send().await?;
        self.auth_token = None;

        Ok(())
    }
}
