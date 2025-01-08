use std::sync::{Arc, LazyLock};

use vulkano::{
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    swapchain::Surface,
    Version, VulkanLibrary,
};
use winit::window::Window;

use super::Renderer;

// FIXME: possible memory leak
// static members don't call "Drop" on program termination,
// so this may cause memory leak, but it's ok as long as OS takes care of it
static LIBRARY: LazyLock<Arc<VulkanLibrary>> =
    LazyLock::new(|| VulkanLibrary::new().expect("Vulkan library is not supported"));

impl Renderer {
    pub fn new(window: Arc<Window>, extensions: InstanceExtensions) -> Self {
        let instance_extensions = InstanceExtensions {
            ext_surface_maintenance1: true,
            ..extensions
        };

        let instance = Self::new_instance(instance_extensions);
        let surface = Surface::from_window(instance.clone(), window.clone())
            .expect("Surface creation failed");

        let physical_device = Self::new_physical_device(instance.clone());
        let (device, queues) =
            Self::create_logical_device(physical_device.clone(), surface.clone());

        let (swapchain, images) = Self::create_swapchain(device.clone(), surface);
        let swapchain_images = Self::zip_image_views(images);

        Self {
            instance,
            physical_device,
            device,
            queues,

            swapchain,
            swapchain_images,
        }
    }

    fn new_instance(enabled_extensions: InstanceExtensions) -> Arc<Instance> {
        let application_version = Version {
            major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        };

        let engine_version = application_version;

        let create_info = InstanceCreateInfo {
            application_name: Some("VoxelEngine".to_string()),
            engine_name: Some("voxen".to_string()),
            application_version,
            engine_version,
            enabled_extensions,
            ..Default::default()
        };

        Instance::new(LIBRARY.clone(), create_info).expect("Instance creation failed")
    }
}
