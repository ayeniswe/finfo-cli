use http_body_util::{BodyExt, Empty};
use hyper::{
    body::{Bytes, Incoming},
    Response,
};
use hyper_tls::HttpsConnector;
use hyper_util::{
    client::legacy::{
        connect::{Connect, HttpConnector},
        Client,
    },
    rt::TokioExecutor,
};
use std::{
    error::Error,
    io::{self, Write as _},
};

pub enum Out {
    STD,
    FILE,
}

pub trait Connection {
    fn new(uri: String) -> Self;

    /// Sends a get request to uri/url
    async fn _get<T: Connect + Clone + Send + Sync + 'static>(
        &self,
        client: &Client<T, Empty<Bytes>>,
        uri: &String,
    ) -> Result<Response<Incoming>, Box<dyn Error + Send + Sync>> {
        let response = client.get(uri.parse()?).await?;
        Ok(response)
    }

    async fn write(
        &self,
        mut response: Response<Incoming>,
        out: Out,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        while let Some(next) = response.frame().await {
            let frame = next?;
            if let Some(chunk) = frame.data_ref() {
                match out {
                    Out::STD => _ = io::stdout().write_all(&chunk),
                    Out::FILE => _ = std::fs::File::create("data/output.txt")?.write_all(&chunk),
                };
            }
        }

        Ok(())
    }
}

pub struct HttpConnection {
    client: Client<HttpConnector, Empty<Bytes>>,
    uri: String,
}

impl Connection for HttpConnection {
    fn new(uri: String) -> Self {
        Self {
            client: Client::builder(TokioExecutor::new()).build_http::<Empty<Bytes>>(),
            uri,
        }
    }
}

impl HttpConnection {
    pub async fn get(&self) -> Result<Response<Incoming>, Box<dyn Error + Send + Sync>> {
        Ok(self._get(&self.client, &self.uri).await?)
    }
}

pub struct HttpsConnection {
    client: Client<HttpsConnector<HttpConnector>, Empty<Bytes>>,
    uri: String,
}

impl Connection for HttpsConnection {
    fn new(uri: String) -> Self {
        Self {
            client: Client::builder(TokioExecutor::new())
                .build::<HttpsConnector<HttpConnector>, Empty<Bytes>>(HttpsConnector::new()),
            uri,
        }
    }
}

impl HttpsConnection {
    pub async fn get(&self) -> Result<Response<Incoming>, Box<dyn Error + Send + Sync>> {
        Ok(self._get(&self.client, &self.uri).await?)
    }
}
