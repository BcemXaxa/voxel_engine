use ash::{
    self,
    vk::{self},
};

pub struct VulkanApplication {
    entry: ash::Entry,
    instance: ash::Instance,
}

impl Drop for VulkanApplication {
    // clean up
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}

impl VulkanApplication {
    pub fn run() {}

    fn new() -> Self {
        let entry = Self::entry();
        let instance = Self::instance(&entry);

        Self { entry, instance }
    }

    fn entry() -> ash::Entry {
        match unsafe { ash::Entry::load() } {
            Ok(entry) => entry,
            Err(error) => panic!("Entry creation failed: {error}"),
        }
    }

    fn instance(entry: &ash::Entry) -> ash::Instance {
        use std::ffi::CString;

        let application_name = CString::new("VoxelEngine").unwrap();
        let engine_name = CString::new("voxen").unwrap();
        let application_version = vk::make_api_version(
            0,
            env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        );
        let engine_version = application_version;

        let application_info = vk::ApplicationInfo {
            p_application_name: application_name.as_ptr(),
            p_engine_name: engine_name.as_ptr(),
            application_version,
            engine_version,
            api_version: vk::API_VERSION_1_3,
            ..vk::ApplicationInfo::default()
        };


        let create_info = vk::InstanceCreateInfo {
            p_application_info: &application_info,
            ..vk::InstanceCreateInfo::default()
        };

        match unsafe { entry.create_instance(&create_info, None) } {
            Ok(instance) => instance,
            Err(error) => panic!("Instance creation failed: {error}"),
        }
    }
}
