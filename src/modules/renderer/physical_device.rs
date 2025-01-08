use std::sync::Arc;

use vulkano::{device::physical::PhysicalDevice, instance::Instance};

use super::Renderer;

impl Renderer {
    pub(super) fn new_physical_device(
        instance: Arc<Instance>,
    ) -> Arc<PhysicalDevice> {
        let physical_devices: Vec<_> = instance
            .enumerate_physical_devices()
            .expect("Physical devices enumeration failed")
            .filter(|physical_device| Self::is_physical_device_suitable(physical_device.clone()))
            .collect();

        physical_devices
            .into_iter()
            .next()
            .expect("No suitable physical devices found")
    }

    pub(super) fn is_physical_device_suitable(physical_device: Arc<PhysicalDevice>) -> bool {
        let _properties = physical_device.properties();
        let mut _has_properties = true;
        //has_properties &= properties.device_type == PhysicalDeviceType::DiscreteGpu;

        let features = physical_device.supported_features();
        let mut has_features = true;
        has_features &= features.geometry_shader;

        let extensions = physical_device.supported_extensions();
        let mut has_extensions = true;
        has_extensions &= extensions.khr_swapchain;

        // TODO: make score system (optional)
        // TODO: make list of missing properties & features & extensions (optional)

        _has_properties && has_features && has_extensions
    }
}
