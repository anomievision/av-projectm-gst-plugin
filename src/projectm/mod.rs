use gst::glib;
use gst::prelude::*;

mod imp;

glib::wrapper! {
    pub struct ProjectM(ObjectSubclass<imp::ProjectM>) 
        @extends  gst_video::VideoSink, gst_base::BaseSink, gst::Element, gst::Object;
}

// GStreamer elements need to be thread-safe. For the private implementation this is automatically
// enforced but for the public wrapper type we need to specify this manually.
unsafe impl Send for ProjectM {}
unsafe impl Sync for ProjectM {}

// Registers the type for our element, and then registers in GStreamer under
// the name "gstProjectM" for being able to instantiate it via e.g.
// gst::ElementFactory::make().
pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    gst::Element::register(
        Some(plugin),
        "projectm",
        gst::Rank::None,
        ProjectM::static_type(),
    )
}
