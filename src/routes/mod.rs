use poem::Route;
use poem_openapi::{OpenApi, OpenApiService};

use crate::config::Config;

use self::v2::ApiV2;

mod v2;

pub(crate) trait BaseApi {
    /// Returns the title of the API.
    fn title() -> &'static str;

    /// Returns the version of the API.
    fn version() -> &'static str;

    /// Returns the route of the API.
    fn route() -> &'static str;

    /// Returns the UI route of the API.
    fn ui_route() -> &'static str;

    /// Returns the API as a nested route.
    fn nest(config: &Config) -> Route
    where
        Self: Sized + OpenApi + Default,
    {
        let api = OpenApiService::new(Self::default(), Self::title(), Self::version())
            .server(config.public_address.clone() + Self::route());
        let ui = api.stoplight_elements();
        Route::new()
            .nest(Self::route(), api)
            .nest(Self::ui_route(), ui)
    }
}

pub(crate) fn route(config: &Config) -> Route {
    Route::new().nest("/", ApiV2::nest(config))
}
