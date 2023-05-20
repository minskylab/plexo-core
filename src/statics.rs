use poem::{
    error::StaticFileError, web::StaticFileRequest, Body, Endpoint, FromRequest, IntoResponse,
    Request, Response, Result,
};
use reqwest::{
    header::{self, LOCATION},
    Method, StatusCode,
};
use std::{
    ffi::OsStr,
    fmt::Write,
    path::{Path, PathBuf},
};

use crate::{commons::authorization::get_token_from_raw_cookie, system::core::Engine};

struct DirectoryTemplate<'a> {
    path: &'a str,
    files: Vec<FileRef>,
}

impl<'a> DirectoryTemplate<'a> {
    fn render(&self) -> String {
        let mut s = format!(
            r#"
        <html>
            <head>
            <title>Index of {}</title>
        </head>
        <body>
        <h1>Index of /{}</h1>
        <ul>"#,
            self.path, self.path
        );

        for file in &self.files {
            if file.is_dir {
                let _ = write!(
                    s,
                    r#"<li><a href="{}">{}/</a></li>"#,
                    file.url, file.filename
                );
            } else {
                let _ = write!(
                    s,
                    r#"<li><a href="{}">{}</a></li>"#,
                    file.url, file.filename
                );
            }
        }

        s.push_str(
            r#"</ul>
        </body>
        </html>"#,
        );

        s
    }
}

struct FileRef {
    url: String,
    filename: String,
    is_dir: bool,
}
pub struct StaticServer {
    path: PathBuf,
    show_files_listing: bool,
    index_file: Option<String>,
    prefer_utf8: bool,
    redirect_to_slash: bool,
    plexo_engine: Engine,
}

impl StaticServer {
    pub fn new(path: impl Into<PathBuf>, plexo_engine: Engine) -> Self {
        Self {
            path: path.into(),
            show_files_listing: false,
            index_file: None,
            prefer_utf8: true,
            redirect_to_slash: false,
            plexo_engine,
        }
    }

    #[must_use]
    pub fn show_files_listing(self) -> Self {
        Self {
            show_files_listing: true,
            ..self
        }
    }

    #[must_use]
    pub fn index_file(self, index: impl Into<String>) -> Self {
        Self {
            index_file: Some(index.into()),
            ..self
        }
    }

    #[must_use]
    pub fn prefer_utf8(self, value: bool) -> Self {
        Self {
            prefer_utf8: value,
            ..self
        }
    }

    #[must_use]
    pub fn redirect_to_slash_directory(self) -> Self {
        Self {
            redirect_to_slash: true,
            ..self
        }
    }
}

#[async_trait::async_trait]
impl Endpoint for StaticServer {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        if req.method() != Method::GET {
            return Err(StaticFileError::MethodNotAllowed(req.method().clone()).into());
        }

        if !req.uri().path().ends_with("login") {
            let Some(auth_token) = req
                .header("Cookie")
                .and_then(get_token_from_raw_cookie)
                 else {
                    return Ok(Response::builder()
                        .status(StatusCode::MOVED_PERMANENTLY)
                        .header(LOCATION, "/login")
                        .header(header::CACHE_CONTROL, "no-cache")
                        .body(Body::empty()));
                };

            let Ok(_member_id) = self
                .plexo_engine
                .auth
                .extract_claims(&auth_token)
                .map(|token_claims| token_claims.member_id()) else {
                    return Ok(Response::builder()
                        .status(StatusCode::MOVED_PERMANENTLY)
                        .header(LOCATION, "/login")
                        .header(header::CACHE_CONTROL, "no-cache")
                        .body(Body::empty()));
                };
        }

        let path = req
            .uri()
            .path()
            .trim_start_matches('/')
            .trim_end_matches('/');

        let path = percent_encoding::percent_decode_str(path)
            .decode_utf8()
            .map_err(|_| StaticFileError::InvalidPath)?;

        let mut file_path = self.path.clone();
        for p in Path::new(&*path) {
            if p == OsStr::new(".") {
                continue;
            } else if p == OsStr::new("..") {
                file_path.pop();
            } else {
                file_path.push(p);
            }
        }

        if !file_path.starts_with(&self.path) {
            return Err(StaticFileError::Forbidden(file_path.display().to_string()).into());
        }

        if file_path.with_extension("html").exists() {
            return Ok(StaticFileRequest::from_request_without_body(&req)
                .await?
                .create_response(file_path.with_extension("html"), self.prefer_utf8)?
                .into_response());
        }

        if !file_path.exists() {
            return Err(StaticFileError::NotFound.into());
        }

        if file_path.is_file() {
            return Ok(StaticFileRequest::from_request_without_body(&req)
                .await?
                .create_response(&file_path, self.prefer_utf8)?
                .into_response());
        } else {
            if self.redirect_to_slash
                && !req.original_uri().path().ends_with('/')
                && (self.index_file.is_some() || self.show_files_listing)
            {
                let redirect_to = format!("{}/", req.original_uri().path());
                return Ok(Response::builder()
                    .status(StatusCode::FOUND)
                    .header(LOCATION, redirect_to)
                    .header(header::CACHE_CONTROL, "no-cache")
                    .finish());
            }

            if let Some(index_file) = &self.index_file {
                let index_path = file_path.join(index_file);
                if index_path.is_file() {
                    return Ok(StaticFileRequest::from_request_without_body(&req)
                        .await?
                        .create_response(&index_path, self.prefer_utf8)?
                        .into_response());
                }
            }

            if self.show_files_listing {
                let read_dir = file_path.read_dir().map_err(StaticFileError::Io)?;
                let mut template = DirectoryTemplate {
                    path: &path,
                    files: Vec::new(),
                };

                for res in read_dir {
                    let entry = res.map_err(StaticFileError::Io)?;

                    if let Some(filename) = entry.file_name().to_str() {
                        let mut base_url = req.original_uri().path().to_string();
                        if !base_url.ends_with('/') {
                            base_url.push('/');
                        }
                        template.files.push(FileRef {
                            url: format!("{}{}", base_url, filename),
                            filename: filename.to_string(),
                            is_dir: entry.path().is_dir(),
                        });
                    }
                }

                let html = template.render();
                Ok(Response::builder()
                    .header(header::CONTENT_TYPE, mime::TEXT_HTML_UTF_8.as_ref())
                    .header(header::CACHE_CONTROL, "no-cache")
                    .body(Body::from_string(html)))
            } else {
                Err(StaticFileError::NotFound.into())
            }
        }
    }
}
