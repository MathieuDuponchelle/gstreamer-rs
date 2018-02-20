use RTSPMediaFactory;
use ffi;
use glib::object::IsA;
use glib::StaticType;
use glib::translate::*;
use glib::value::ToSendValue;
use std::ptr;

pub trait RTSPMediaFactoryExtManual {
    fn add_role<'a, T: 'static + GlibPtrDefault + ToGlibPtr<'a, <T as GlibPtrDefault>::GlibType> + StaticType>(&self, role: &str, v: T);
}

impl<O: IsA<RTSPMediaFactory>> RTSPMediaFactoryExtManual for O {
    fn add_role<'a, T: 'static + GlibPtrDefault + ToGlibPtr<'a, <T as GlibPtrDefault>::GlibType> + StaticType>(&self, role: &str, v: T) {
        unsafe {
            ffi::gst_rtsp_media_factory_add_role(self.to_glib_none().0, role.to_glib_none().0, T::static_type().to_glib() as *const _, v.to_glib_none().0, ptr::null_mut::<u8>());
        }
    }
}
