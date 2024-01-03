// Links to resources about the Fetch API, and how to use it with wasm-bindgen:
// * https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API
// * https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
// * https://docs.rs/web-sys/latest/web_sys/struct.Request.html#
// * https://docs.rs/web-sys/latest/web_sys/struct.RequestInit.html#

#[cfg(feature = "bin")]
use core::convert::Infallible;

use alloc::string::{String, ToString};
use web_sys::wasm_bindgen::JsValue;
use web_sys::{Headers, RequestCache, RequestCredentials, RequestInit, RequestMode, RequestRedirect};

pub trait IntoBody {
    type Error;

    fn content_type(&self) -> &'static str;

    fn to_js(&self) -> Result<JsValue, Self::Error>;
}

#[cfg(feature = "json")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "json")))]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Json<T: serde::Serialize>(T);

#[cfg(feature = "json")]
impl<T: serde::Serialize> IntoBody for Json<T> {
    type Error = serde_json::Error;

    #[inline]
    fn content_type(&self) -> &'static str {
        "application/json"
    }

    #[inline]
    fn to_js(&self) -> Result<JsValue, Self::Error> {
        let json = serde_json::to_string(&self.0)?;
        Ok(JsValue::from_str(&json))
    }
}

#[cfg(feature = "bin")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "bin")))]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub struct Bin<T: AsRef<[u8]>>(T);

#[cfg(feature = "bin")]
impl<T: AsRef<[u8]>> IntoBody for Bin<T> {
    type Error = Infallible;

    #[inline]
    fn content_type(&self) -> &'static str {
        "application/octet-stream"
    }

    #[inline]
    fn to_js(&self) -> Result<JsValue, Self::Error> {
        Ok(web_sys::js_sys::Uint8Array::from(self.0.as_ref()).buffer().into())
    }
}

/// The [`Cache`] setting of a [`Fetch`] request controls how the request will interact with the browser's HTTP cache.
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
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

/// The [`Credentials`] setting of a [`Fetch`] request indicates whether the user agent should send or receive cookies from the other domain in the case of cross-origin requests.
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub enum Credentials {
    /// Never send or receive cookies.
    Omit,
    /// Send user credentials (cookies, basic http auth, etc..) if the URL is on the same origin as the calling script. This is the default value.
    #[default]
    SameOrigin,
    /// Always send user credentials (cookies, basic http auth, etc..), even for cross-origin calls.
    Include,
}

/// The [`Method`] setting of a [`Fetch`] request defines the HTTP verb to use.
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
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
    /// Some other, as of yet unknown HTTP method.
    Other(&'static str),
}

/// The [`Mode`] setting of a [`Fetch`] request is used to determine if cross-origin requests lead to valid responses, and which properties of the response are readable.
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub enum Mode {
    #[default]
    /// If a request is made to another origin with this mode set, the result is an error. You could use this to ensure that a request is always being made to your origin.
    SameOrigin,
    /// Prevents the method from being anything other than HEAD, GET or POST, and the headers from being anything other than CORS-safelisted request headers. If any ServiceWorkers intercept these requests, they may not add or override any headers except for those that are CORS-safelisted request headers. In addition, JavaScript may not access any properties of the resulting Response. This ensures that ServiceWorkers do not affect the semantics of the Web and prevents security and privacy issues arising from leaking data across domains.
    NoCors,
    /// Allows cross-origin requests, for example to access various APIs offered by 3rd party vendors. These are expected to adhere to the CORS protocol. Only a limited set of headers are exposed in the Response, but the body is readable.
    Cors,
    /// A mode for supporting navigation. The navigate value is intended to be used only by HTML navigation. A navigate request is created only while navigating between documents.
    Navigate,
}

/// The [`Redirect`] setting of a [`Fetch`] request contains the mode for how redirects are handled.
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub enum Redirect {
    /// Automatically follow redirects. Unless otherwise stated the redirect mode is set to follow.
    #[default]
    Follow,
    /// Abort with an error if a redirect occurs.
    Error,
    /// Caller intends to process the response in another context. See WHATWG fetch standard for more information.
    Manual,
}

/// The [`ReferrerPolicy`] setting of a [`Fetch`] request controls how much referrer information (sent with the `Referer` header) should be included with requests. Aside from the HTTP header, you can set this policy in HTML.
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Debug)]
pub enum ReferrerPolicy {
    /// The `Referer` header will be omitted: sent requests do not include any referrer information.
    NoReferrer,
    /// Send the `origin`, `path`, and `querystring` in `Referer` when the protocol security level stays the same or improves (HTTP to HTTP, HTTP to HTTPS, HTTPS to HTTPS). Don't send the Referer header for requests to less secure destinations (HTTPS to HTTP, HTTPS to file).
    NoReferrerWhenDowngrade,
    /// Send only the `origin` in the `Referer` header. For example, a document at `https://example.com/page.html` will send the referrer `https://example.com/`.
    Origin,
    /// When performing a [`Mode::SameOrigin`] request to the same protocol level (HTTP to HTTP, HTTPS to HTTPS), send the `origin`, `path`, and `querystring`. Send only the `origin` for cross origin requests and requests to less secure destinations (HTTPS to HTTP).
    OriginWhenCrossOrigin,
    /// Send the `origin`, `path`, and `querystring` when performing any request, regardless of security.
    UnsafeUrl,
    /// Send the origin, path, and `querystring` for [`Mode::SameOrigin`] requests. Don't send the Referer header for cross-origin requests.
    SameOrigin,
    /// Send only the `origin` when the protocol security level stays the same (HTTPS to HTTPS). Don't send the `Referer` header to less secure destinations (HTTPS to HTTP).
    StrictOrigin,
    #[default]
    /// Send the `origin`, `path`, and `querystring` when performing a [`Mode::SameOrigin`] request. For cross-origin requests send the `origin` (only) when the protocol security level stays the same (HTTPS to HTTPS). Don't send the `Referer` header to less secure destinations (HTTPS to HTTP).
    StrictOriginWhenCrossOrigin,
}

#[derive(Debug)]
pub struct Fetch {
    input: String,
    headers: Headers,
    init: RequestInit,
}

impl Fetch {
    #[inline]
    pub fn new<U: ToString>(method: Method, url: U) -> Self {
        let mut init = RequestInit::new();
        init.method(match method {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
            Method::Patch => "PATCH",
            Method::Other(other) => other,
        });

        let headers = Headers::new().unwrap();
        init.headers(&headers);

        Self {
            init,
            headers,
            input: url.to_string(),
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
    pub fn input(&self) -> &str {
        &self.input
    }

    #[inline]
    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    #[inline]
    pub fn init(&self) -> &RequestInit {
        &self.init
    }

    #[inline]
    pub fn with_body<B: IntoBody>(&mut self, body: B) -> Result<&mut Self, B::Error> {
        self.headers.set("Content-Type", body.content_type()).unwrap();
        self.init.body(Some(&body.to_js()?));
        Ok(self)
    }

    #[inline]
    pub fn with_cache(&mut self, cache: Cache) -> &mut Self {
        self.init.cache(match cache {
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
    pub fn with_credentials(&mut self, credentials: Credentials) -> &mut Self {
        self.init.credentials(match credentials {
            Credentials::Omit => RequestCredentials::Omit,
            Credentials::SameOrigin => RequestCredentials::SameOrigin,
            Credentials::Include => RequestCredentials::Include,
        });
        self
    }

    #[inline]
    pub fn with_integrity<S: AsRef<str>>(&mut self, integrity: S) -> &mut Self {
        self.init.integrity(integrity.as_ref());
        self
    }

    #[inline]
    pub fn with_mode(&mut self, mode: Mode) -> &mut Self {
        self.init.mode(match mode {
            Mode::SameOrigin => RequestMode::SameOrigin,
            Mode::NoCors => RequestMode::NoCors,
            Mode::Cors => RequestMode::Cors,
            Mode::Navigate => RequestMode::Navigate,
        });
        self
    }

    #[inline]
    pub fn with_redirect(&mut self, redirect: Redirect) -> &mut Self {
        self.init.redirect(match redirect {
            Redirect::Follow => RequestRedirect::Follow,
            Redirect::Error => RequestRedirect::Error,
            Redirect::Manual => RequestRedirect::Manual,
        });
        self
    }

    #[inline]
    pub fn with_referrer<S: AsRef<str>>(&mut self, referrer: S) -> &mut Self {
        self.init.referrer(referrer.as_ref());
        self
    }

    #[inline]
    pub fn with_referrer_policy(&mut self, referrer_policy: ReferrerPolicy) -> &mut Self {
        self.init.referrer_policy(match referrer_policy {
            ReferrerPolicy::NoReferrer => web_sys::ReferrerPolicy::NoReferrer,
            ReferrerPolicy::NoReferrerWhenDowngrade => web_sys::ReferrerPolicy::NoReferrerWhenDowngrade,
            ReferrerPolicy::Origin => web_sys::ReferrerPolicy::Origin,
            ReferrerPolicy::OriginWhenCrossOrigin => web_sys::ReferrerPolicy::OriginWhenCrossOrigin,
            ReferrerPolicy::UnsafeUrl => web_sys::ReferrerPolicy::UnsafeUrl,
            ReferrerPolicy::SameOrigin => web_sys::ReferrerPolicy::SameOrigin,
            ReferrerPolicy::StrictOrigin => web_sys::ReferrerPolicy::StrictOrigin,
            ReferrerPolicy::StrictOriginWhenCrossOrigin => web_sys::ReferrerPolicy::StrictOriginWhenCrossOrigin,
        });
        self
    }

    #[inline]
    pub async fn execute(&self) {
        let _res = web_sys::window()
            .unwrap()
            .fetch_with_str_and_init(&self.input, &self.init);
        todo!()
    }
}
