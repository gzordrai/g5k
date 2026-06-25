use reqwest::{Client, Method, Response, header::LOCATION};
use secrecy::{ExposeSecret, SecretString};
use serde::{Serialize, de::DeserializeOwned};

use crate::error::{G5KError, Result};

macro_rules! endpoint {
    (
        $method:ident, $name:ident, $path:literal, ($($parg:ident : $pty:ty),*), Location
    ) => {
        pub async fn $name(&self, $($parg: $pty,)*) -> Result<String> {
            self.request_location(Method::$method, format!($path), None::<&()>, None::<&()>).await
        }
    };

    (
        $method:ident, $name:ident, $path:literal, ($($parg:ident : $pty:ty),*), body: $bty:ty, Location
    ) => {
        pub async fn $name(&self, $($parg: $pty,)* body: &$bty) -> Result<String> {
            self.request_location(Method::$method, format!($path), None::<&()>, Some(body)).await
        }
    };

    (
        $method:ident, $name:ident, $path:literal, ($($parg:ident : $pty:ty),*), $ret:ty
    ) => {
        pub async fn $name(&self, $($parg: $pty,)*) -> Result<$ret> {
            let path = format!($path);
            self.request(Method::$method, &path, None::<&()>, None::<&()>).await
        }
    };

    (
        $method:ident, $name:ident, $path:literal, ($($parg:ident : $pty:ty),*), query: $qty:ty, $ret:ty
    ) => {
        pub async fn $name(&self, $($parg: $pty,)* query: &$qty) -> Result<$ret> {
            let path = format!($path);
            self.request(Method::$method, &path, Some(query), None::<&()>).await
        }
    };

    (
        $method:ident, $name:ident, $path:literal, ($($parg:ident : $pty:ty),*), body: $bty:ty, $ret:ty
    ) => {
        pub async fn $name(&self, $($parg: $pty,)* body: &$bty) -> Result<$ret> {
            let path = format!($path);
            self.request(Method::$method, &path, None::<&()>, Some(body)).await
        }
    };

    (
        $method:ident, $name:ident, $path:literal, ($($parg:ident : $pty:ty),*), query: $qty:ty, body: $bty:ty, $ret:ty
    ) => {
        pub async fn $name(&self, $($parg: $pty,)* query: &$qty, body: &$bty) -> Result<$ret> {
            let path = format!($path);
            self.request(Method::$method, &path, Some(query), Some(body)).await
        }
    };
}

pub struct G5KClient {
    http: Client,
    username: SecretString,
    password: SecretString,
}

impl G5KClient {
    const BASE_URL: &str = "https://api.grid5000.fr/stable";

    pub fn new<U, P>(username: U, password: P) -> Self
    where
        U: Into<SecretString>,
        P: Into<SecretString>,
    {
        Self {
            http: Client::new(),
            username: username.into(),
            password: password.into(),
        }
    }

    async fn send<P, Q, B>(
        &self,
        method: Method,
        path: P,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<Response>
    where
        P: AsRef<str>,
        Q: Serialize,
        B: Serialize,
    {
        let url = format!("{}{}", Self::BASE_URL, path.as_ref());
        let mut req = self.http.request(method, url).basic_auth(
            self.username.expose_secret(),
            Some(self.password.expose_secret()),
        );

        if let Some(q) = query {
            req = req.query(q);
        }

        if let Some(b) = body {
            req = req.json(b);
        }

        let res = req.send().await?;
        let status = res.status();

        if !status.is_success() {
            let body = res.text().await.unwrap_or_default();

            return Err(G5KError::Api { status, body });
        }

        Ok(res)
    }

    async fn request<P, Q, B, T>(
        &self,
        method: Method,
        path: P,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<T>
    where
        P: AsRef<str>,
        Q: Serialize,
        B: Serialize,
        T: DeserializeOwned,
    {
        let res = self.send(method, path, query, body).await?;

        Ok(res.json::<T>().await?)
    }

    async fn request_location<P, Q, B>(
        &self,
        method: Method,
        path: P,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<String>
    where
        P: AsRef<str>,
        Q: Serialize,
        B: Serialize,
    {
        let res = self.send(method, path, query, body).await?;

        res.headers()
            .get(LOCATION)
            .ok_or(G5KError::MissingLocation)?
            .to_str()
            .map(|s| s.to_string())
            .map_err(|_| G5KError::InvalidLocation)
    }
}
