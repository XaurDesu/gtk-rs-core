// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AppInfoCreateFlags;
use crate::AppLaunchContext;
#[cfg(any(feature = "v2_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
use crate::AsyncResult;
#[cfg(any(feature = "v2_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
use crate::Cancellable;
use crate::File;
use crate::Icon;
use glib::object::IsA;
use glib::translate::*;
#[cfg(any(feature = "v2_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GAppInfo")]
    pub struct AppInfo(Interface<ffi::GAppInfo, ffi::GAppInfoIface>);

    match fn {
        type_ => || ffi::g_app_info_get_type(),
    }
}

impl AppInfo {
    pub const NONE: Option<&'static AppInfo> = None;

    #[doc(alias = "g_app_info_create_from_commandline")]
    pub fn create_from_commandline(
        commandline: impl AsRef<std::ffi::OsStr>,
        application_name: Option<&str>,
        flags: AppInfoCreateFlags,
    ) -> Result<AppInfo, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_app_info_create_from_commandline(
                commandline.as_ref().to_glib_none().0,
                application_name.to_glib_none().0,
                flags.into_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_app_info_get_all")]
    #[doc(alias = "get_all")]
    pub fn all() -> Vec<AppInfo> {
        unsafe { FromGlibPtrContainer::from_glib_full(ffi::g_app_info_get_all()) }
    }

    #[doc(alias = "g_app_info_get_all_for_type")]
    #[doc(alias = "get_all_for_type")]
    pub fn all_for_type(content_type: &str) -> Vec<AppInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_app_info_get_all_for_type(
                content_type.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_app_info_get_default_for_type")]
    #[doc(alias = "get_default_for_type")]
    pub fn default_for_type(content_type: &str, must_support_uris: bool) -> Option<AppInfo> {
        unsafe {
            from_glib_full(ffi::g_app_info_get_default_for_type(
                content_type.to_glib_none().0,
                must_support_uris.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_app_info_get_default_for_uri_scheme")]
    #[doc(alias = "get_default_for_uri_scheme")]
    pub fn default_for_uri_scheme(uri_scheme: &str) -> Option<AppInfo> {
        unsafe {
            from_glib_full(ffi::g_app_info_get_default_for_uri_scheme(
                uri_scheme.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_app_info_get_fallback_for_type")]
    #[doc(alias = "get_fallback_for_type")]
    pub fn fallback_for_type(content_type: &str) -> Vec<AppInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_app_info_get_fallback_for_type(
                content_type.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_app_info_get_recommended_for_type")]
    #[doc(alias = "get_recommended_for_type")]
    pub fn recommended_for_type(content_type: &str) -> Vec<AppInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_app_info_get_recommended_for_type(
                content_type.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_app_info_launch_default_for_uri")]
    pub fn launch_default_for_uri(
        uri: &str,
        context: Option<&impl IsA<AppLaunchContext>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_app_info_launch_default_for_uri(
                uri.to_glib_none().0,
                context.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == 0, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    #[doc(alias = "g_app_info_launch_default_for_uri_async")]
    pub fn launch_default_for_uri_async<P: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        uri: &str,
        context: Option<&impl IsA<AppLaunchContext>>,
        cancellable: Option<&impl IsA<Cancellable>>,
        callback: P,
    ) {
        let user_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn launch_default_for_uri_async_trampoline<
            P: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_app_info_launch_default_for_uri_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<P> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = launch_default_for_uri_async_trampoline::<P>;
        unsafe {
            ffi::g_app_info_launch_default_for_uri_async(
                uri.to_glib_none().0,
                context.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(any(feature = "v2_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_50")))]
    pub fn launch_default_for_uri_future(
        uri: &str,
        context: Option<&(impl IsA<AppLaunchContext> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let uri = String::from(uri);
        let context = context.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(
            &(),
            move |_obj, cancellable, send| {
                Self::launch_default_for_uri_async(
                    &uri,
                    context.as_ref().map(::std::borrow::Borrow::borrow),
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    #[doc(alias = "g_app_info_reset_type_associations")]
    pub fn reset_type_associations(content_type: &str) {
        unsafe {
            ffi::g_app_info_reset_type_associations(content_type.to_glib_none().0);
        }
    }
}

pub trait AppInfoExt: 'static {
    #[doc(alias = "g_app_info_add_supports_type")]
    fn add_supports_type(&self, content_type: &str) -> Result<(), glib::Error>;

    #[doc(alias = "g_app_info_can_delete")]
    fn can_delete(&self) -> bool;

    #[doc(alias = "g_app_info_can_remove_supports_type")]
    fn can_remove_supports_type(&self) -> bool;

    #[doc(alias = "g_app_info_delete")]
    fn delete(&self) -> bool;

    #[doc(alias = "g_app_info_dup")]
    fn dup(&self) -> AppInfo;

    #[doc(alias = "g_app_info_equal")]
    fn equal(&self, appinfo2: &impl IsA<AppInfo>) -> bool;

    #[doc(alias = "g_app_info_get_commandline")]
    #[doc(alias = "get_commandline")]
    fn commandline(&self) -> Option<std::path::PathBuf>;

    #[doc(alias = "g_app_info_get_description")]
    #[doc(alias = "get_description")]
    fn description(&self) -> Option<glib::GString>;

    #[doc(alias = "g_app_info_get_display_name")]
    #[doc(alias = "get_display_name")]
    fn display_name(&self) -> glib::GString;

    #[doc(alias = "g_app_info_get_executable")]
    #[doc(alias = "get_executable")]
    fn executable(&self) -> std::path::PathBuf;

    #[doc(alias = "g_app_info_get_icon")]
    #[doc(alias = "get_icon")]
    fn icon(&self) -> Option<Icon>;

    #[doc(alias = "g_app_info_get_id")]
    #[doc(alias = "get_id")]
    fn id(&self) -> Option<glib::GString>;

    #[doc(alias = "g_app_info_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> glib::GString;

    #[doc(alias = "g_app_info_get_supported_types")]
    #[doc(alias = "get_supported_types")]
    fn supported_types(&self) -> Vec<glib::GString>;

    #[doc(alias = "g_app_info_launch")]
    fn launch(
        &self,
        files: &[File],
        context: Option<&impl IsA<AppLaunchContext>>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_app_info_launch_uris")]
    fn launch_uris(
        &self,
        uris: &[&str],
        context: Option<&impl IsA<AppLaunchContext>>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_app_info_remove_supports_type")]
    fn remove_supports_type(&self, content_type: &str) -> Result<(), glib::Error>;

    #[doc(alias = "g_app_info_set_as_default_for_extension")]
    fn set_as_default_for_extension(
        &self,
        extension: impl AsRef<std::path::Path>,
    ) -> Result<(), glib::Error>;

    #[doc(alias = "g_app_info_set_as_default_for_type")]
    fn set_as_default_for_type(&self, content_type: &str) -> Result<(), glib::Error>;

    #[doc(alias = "g_app_info_set_as_last_used_for_type")]
    fn set_as_last_used_for_type(&self, content_type: &str) -> Result<(), glib::Error>;

    #[doc(alias = "g_app_info_should_show")]
    fn should_show(&self) -> bool;

    #[doc(alias = "g_app_info_supports_files")]
    fn supports_files(&self) -> bool;

    #[doc(alias = "g_app_info_supports_uris")]
    fn supports_uris(&self) -> bool;
}

impl<O: IsA<AppInfo>> AppInfoExt for O {
    fn add_supports_type(&self, content_type: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_app_info_add_supports_type(
                self.as_ref().to_glib_none().0,
                content_type.to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == 0, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn can_delete(&self) -> bool {
        unsafe { from_glib(ffi::g_app_info_can_delete(self.as_ref().to_glib_none().0)) }
    }

    fn can_remove_supports_type(&self) -> bool {
        unsafe {
            from_glib(ffi::g_app_info_can_remove_supports_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn delete(&self) -> bool {
        unsafe { from_glib(ffi::g_app_info_delete(self.as_ref().to_glib_none().0)) }
    }

    fn dup(&self) -> AppInfo {
        unsafe { from_glib_full(ffi::g_app_info_dup(self.as_ref().to_glib_none().0)) }
    }

    fn equal(&self, appinfo2: &impl IsA<AppInfo>) -> bool {
        unsafe {
            from_glib(ffi::g_app_info_equal(
                self.as_ref().to_glib_none().0,
                appinfo2.as_ref().to_glib_none().0,
            ))
        }
    }

    fn commandline(&self) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(ffi::g_app_info_get_commandline(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn description(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_app_info_get_description(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn display_name(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_app_info_get_display_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn executable(&self) -> std::path::PathBuf {
        unsafe {
            from_glib_none(ffi::g_app_info_get_executable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn icon(&self) -> Option<Icon> {
        unsafe { from_glib_none(ffi::g_app_info_get_icon(self.as_ref().to_glib_none().0)) }
    }

    fn id(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::g_app_info_get_id(self.as_ref().to_glib_none().0)) }
    }

    fn name(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::g_app_info_get_name(self.as_ref().to_glib_none().0)) }
    }

    fn supported_types(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_app_info_get_supported_types(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn launch(
        &self,
        files: &[File],
        context: Option<&impl IsA<AppLaunchContext>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_app_info_launch(
                self.as_ref().to_glib_none().0,
                files.to_glib_none().0,
                context.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == 0, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn launch_uris(
        &self,
        uris: &[&str],
        context: Option<&impl IsA<AppLaunchContext>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_app_info_launch_uris(
                self.as_ref().to_glib_none().0,
                uris.to_glib_none().0,
                context.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == 0, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn remove_supports_type(&self, content_type: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_app_info_remove_supports_type(
                self.as_ref().to_glib_none().0,
                content_type.to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == 0, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_as_default_for_extension(
        &self,
        extension: impl AsRef<std::path::Path>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_app_info_set_as_default_for_extension(
                self.as_ref().to_glib_none().0,
                extension.as_ref().to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == 0, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_as_default_for_type(&self, content_type: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_app_info_set_as_default_for_type(
                self.as_ref().to_glib_none().0,
                content_type.to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == 0, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn set_as_last_used_for_type(&self, content_type: &str) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::g_app_info_set_as_last_used_for_type(
                self.as_ref().to_glib_none().0,
                content_type.to_glib_none().0,
                &mut error,
            );
            assert_eq!(is_ok == 0, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn should_show(&self) -> bool {
        unsafe { from_glib(ffi::g_app_info_should_show(self.as_ref().to_glib_none().0)) }
    }

    fn supports_files(&self) -> bool {
        unsafe {
            from_glib(ffi::g_app_info_supports_files(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn supports_uris(&self) -> bool {
        unsafe {
            from_glib(ffi::g_app_info_supports_uris(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for AppInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AppInfo")
    }
}
