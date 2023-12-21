// https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API
// https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
// https://docs.rs/web-sys/latest/web_sys/struct.Request.html#
// https://docs.rs/web-sys/latest/web_sys/struct.RequestInit.html#

use alloc::string::{String, ToString};
use web_sys::{RequestCache, RequestInit, RequestMode};

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum Method {
    /// The `GET` method requests a representation of the specified resource. Requests using `GET` should only retrieve data.
    #[default]
    Get,
    /// The `HEAD` method asks for a response identical to a `GET` request, but without the response body.
    Head,
    /// The `POST` method submits an entity to the specified resource, often causing a change in state or side effects on the server.
    Post,
    /// The `PUT` method replaces all current representations of the target resource with the request payload.
    Put,
    /// The `DELETE` method deletes the specified resource.
    Delete,
    /// The `CONNECT` method establishes a tunnel to the server identified by the target resource.
    Connect,
    /// The `OPTIONS` method describes the communication options for the target resource.
    Options,
    /// The `TRACE` method performs a message loop-back test along the path to the target resource.
    Trace,
    /// The `PATCH` method applies partial modifications to a resource.
    Patch,
    /// Some other, unknown HTTP method.
    Other(&'static str),
}

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum Cache {
    /// The browser looks for a matching request in its HTTP cache:
    /// * if there is a match and it is fresh, it will be returned from the cache.
    /// * if there is a match but it is stale, the browser will make a conditional request to the remote server. If the server indicates that the resource has not changed, it will be returned from the cache. Otherwise the resource will be downloaded from the server and the cache will be updated.
    /// * if there is no match, the browser will make a normal request, and will update the cache with the downloaded resource.
    #[default]
    Default,
    /// The browser fetches the resource from the remote server without first looking in the cache, and will not update the cache with the downloaded resource.
    NoStore,
    /// The browser fetches the resource from the remote server without first looking in the cache, but then will update the cache with the downloaded resource.
    Reload,
    /// The browser looks for a matching request in its HTTP cache:
    /// * if there is a match, fresh or stale, the browser will make a conditional request to the remote server. If the server indicates that the resource has not changed, it will be returned from the cache. Otherwise the resource will be downloaded from the server and the cache will be updated.
    /// * if there is no match, the browser will make a normal request, and will update the cache with the downloaded resource.
    NoCache,
    /// The browser looks for a matching request in its HTTP cache:
    /// * if there is a match, fresh or stale, it will be returned from the cache.
    /// *i f there is no match, the browser will make a normal request, and will update the cache with the downloaded resource.
    ForceCache,
    /// The browser looks for a matching request in its HTTP cache:
    /// * if there is a match, fresh or stale, it will be returned from the cache.
    /// * if there is no match, the browser will respond with a 504 Gateway timeout status.
    /// The [`Cache::OnlyIfCached`] mode can only be used if the request's mode is [`Mode::SameOrigin`]. Cached redirects will be followed if the request's redirect property is "follow" and the redirects do not violate the "same-origin" mode.
    OnlyIfCached,
}

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub enum Mode {
    #[default]
    SameOrigin,
    NoCors,
    Cors,
    Navigate,
}

#[derive(Debug)]
pub struct Fetch {
    url: String,
    opts: RequestInit,
}

impl Fetch {
    #[inline]
    pub fn new<U: ToString>(method: Method, url: U) -> Self {
        let mut opts = RequestInit::new();
        opts.method(match method {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
            Method::Patch => "PATCH",
            Method::Other(method) => method,
        });

        Self {
            opts,
            url: url.to_string(),
        }
    }

    #[inline]
    pub fn get<U: ToString>(url: U) -> Self {
        Self::new(Method::Get, url)
    }

    #[inline]
    pub fn post<U: ToString>(url: U) -> Self {
        Self::new(Method::Post, url)
    }

    #[inline]
    pub fn head<U: ToString>(url: U) -> Self {
        Self::new(Method::Head, url)
    }

    #[inline]
    pub fn put<U: ToString>(url: U) -> Self {
        Self::new(Method::Put, url)
    }

    #[inline]
    pub fn delete<U: ToString>(url: U) -> Self {
        Self::new(Method::Delete, url)
    }

    #[inline]
    pub fn patch<U: ToString>(url: U) -> Self {
        Self::new(Method::Patch, url)
    }

    #[inline]
    pub fn with_cache(mut self, cache: Cache) -> Self {
        self.opts.cache(match cache {
            Cache::Default => RequestCache::Default,
            Cache::NoStore => RequestCache::NoStore,
            Cache::Reload => RequestCache::Reload,
            Cache::NoCache => RequestCache::NoCache,
            Cache::ForceCache => RequestCache::ForceCache,
            Cache::OnlyIfCached => RequestCache::OnlyIfCached,
        });
        self
    }

    #[inline]
    pub fn with_mode(mut self, mode: Mode) -> Self {
        self.opts.mode(match mode {
            Mode::SameOrigin => RequestMode::SameOrigin,
            Mode::NoCors => RequestMode::NoCors,
            Mode::Cors => RequestMode::Cors,
            Mode::Navigate => RequestMode::Navigate,
        });
        self
    }

    // #[inline]
    // pub fn with_body(self) {

    // }

    #[inline]
    pub fn execute(self, url: &str) {
        let opts = RequestInit::new();
        todo!()
    }
}
