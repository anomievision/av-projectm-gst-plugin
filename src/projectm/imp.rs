use gst::glib;
use gst::prelude::*;
use gst::subclass::prelude::*;
use gst_base::subclass::prelude::*;
use gst_video::subclass::prelude::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;

// This module contains the private implementation details of our element
static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
    gst::DebugCategory::new(
        "ProjectM",
        gst::DebugColorFlags::empty(),
        Some("ProjectM Visualizer Renderer"),
    )
});

// Default values of properties
const DEFAULT_MESH_X: u32 = 96;
const DEFAULT_MESH_Y: u32 = 54;
const DEFAULT_FPS: u32 = 30;
const DEFAULT_TEXTURE_SIZE: u32 = 512;
const DEFAULT_WINDOW_WIDTH: u32 = 1280;
const DEFAULT_WINDOW_HEIGHT: u32 = 720;
const DEFAULT_PRESET_DURATION: f64 = 3.0;
const DEFAULT_SOFT_CUT_DURATION: f64 = 15.0;
const DEFAULT_HARD_CUT_DURATION: f64 = 60.0;
const DEFAULT_HARD_CUT_ENABLED: bool = false;
const DEFAULT_HARD_CUT_SENSITIVITY: f32 = 0.0;
const DEFAULT_BEAT_SENSITIVITY: f32 = 0.5;
const DEFAULT_ASPECT_CORRECTION: bool = true;
const DEFAULT_EASTER_EGG: f32 = 0.5;

// Property value storage
#[derive(Debug, Clone, Copy)]
struct Settings {
    pub mesh_x: u32,
    pub mesh_y: u32,
    pub fps: u32,
    pub texture_size: u32,
    pub window_width: u32,
    pub window_height: u32,
    // pub texture_path: String,
    // pub data_path: String,
    pub preset_duration: f64,
    pub soft_cut_duration: f64,
    pub hard_cut_duration: f64,
    pub hard_cut_enabled: bool,
    pub hard_cut_sensitivity: f32,
    pub beat_sensitivity: f32,
    pub aspect_correction: bool,
    pub easter_egg: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            mesh_x: DEFAULT_MESH_X,
            mesh_y: DEFAULT_MESH_Y,
            fps: DEFAULT_FPS,
            texture_size: DEFAULT_TEXTURE_SIZE,
            window_width: DEFAULT_WINDOW_WIDTH,
            window_height: DEFAULT_WINDOW_HEIGHT,
            preset_duration: DEFAULT_PRESET_DURATION,
            soft_cut_duration: DEFAULT_SOFT_CUT_DURATION,
            hard_cut_duration: DEFAULT_HARD_CUT_DURATION,
            hard_cut_enabled: DEFAULT_HARD_CUT_ENABLED,
            hard_cut_sensitivity: DEFAULT_HARD_CUT_SENSITIVITY,
            beat_sensitivity: DEFAULT_BEAT_SENSITIVITY,
            aspect_correction: DEFAULT_ASPECT_CORRECTION,
            easter_egg: DEFAULT_EASTER_EGG,
        }
    }
}

// Struct containing all the element data
#[derive(Default)]
pub struct ProjectM {
    settings: Mutex<Settings>,
}

impl ProjectM {}

// This trait registers our type with the GObject object system and
// provides the entry points for creating a new instance and setting
// up the class data
#[glib::object_subclass]
impl ObjectSubclass for ProjectM {
    const NAME: &'static str = "ProjectM";
    type Type = super::ProjectM;
    type ParentType = gst_video::VideoSink;
}

// Implementation of glib::Object virtual methods
impl ObjectImpl for ProjectM {
    fn properties() -> &'static [glib::ParamSpec] {
        // Metadata for the properties
        static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
            vec![
                glib::ParamSpecUInt::builder("meshx")
                    .nick("MeshX")
                    .blurb("MeshXBlurb")
                    .default_value(DEFAULT_MESH_X)
                    .mutable_playing()
                    .build(),
                glib::ParamSpecUInt::builder("meshy")
                    .nick("MeshY")
                    .blurb("MeshYBlurb")
                    .default_value(DEFAULT_MESH_Y)
                    .mutable_playing()
                    .build(),
            ]
        });

        PROPERTIES.as_ref()
    }

    // Called whenever a value of a property is changed. It can be called
    // at any time from any thread.
    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
        match pspec.name() {
            "meshx" => {
                let mut settings = self.settings.lock().unwrap();
                let mesh_x = value.get().expect("type checked upstream");
                gst::info!(
                    CAT,
                    imp: self,
                    "Changing meshx from {} to {}",
                    settings.mesh_x,
                    mesh_x
                );
                settings.mesh_x = mesh_x;
            }
            "meshy" => {
                let mut settings = self.settings.lock().unwrap();
                let mesh_y = value.get().expect("type checked upstream");
                gst::info!(
                    CAT,
                    imp: self,
                    "Changing meshy from {} to {}",
                    settings.mesh_y,
                    mesh_y
                );
                settings.mesh_y = mesh_y;
            }
            _ => unimplemented!(),
        }
    }

    // Called whenever a value of a property is read. It can be called
    // at any time from any thread.
    fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
        match pspec.name() {
            "meshx" => {
                let settings = self.settings.lock().unwrap();
                settings.mesh_x.to_value()
            }
            "meshy" => {
                let settings = self.settings.lock().unwrap();
                settings.mesh_y.to_value()
            }
            _ => unimplemented!(),
        }
    }
}

impl GstObjectImpl for ProjectM {}

impl ElementImpl for ProjectM {
    // Set the element specific metadata. This information is what
    // is visible from gst-inspect-1.0 and can also be programatically
    // retrieved from the gst::Registry after initial registration
    // without having to load the plugin in memory.
    fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
        static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
            gst::subclass::ElementMetadata::new(
                "ProjectM Visualizer Renderer",
                "Sink/Generator/Video",
                "Generates a visual rendering in a video/x-raw from an audio/x-raw",
                "AnomieVision <anomievision@gmail.com.com>",
            )
        });

        Some(&*ELEMENT_METADATA)
    }
}

impl BaseSinkImpl for ProjectM {}

impl VideoSinkImpl for ProjectM {}
