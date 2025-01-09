use std::sync::Arc;

use vulkano::{
    device::{physical::PhysicalDevice, Queue, QueueFamilyProperties, QueueFlags},
    swapchain::Surface,
};

type PresentSupport = bool;
pub(super) struct Queues {
    graphics_queues: Vec<(Arc<Queue>, QueueFlags, PresentSupport)>,
    compute_queues: Vec<(Arc<Queue>, QueueFlags, PresentSupport)>,
    transfer_queues: Vec<(Arc<Queue>, QueueFlags, PresentSupport)>,
}

impl Queues {
    pub(super) fn new(
        queues: Vec<Arc<Queue>>,
        families_properties: &[QueueFamilyProperties],
        device: Arc<PhysicalDevice>,
        surface: &Surface,
    ) -> Self {
        let mut graphics_queues = Vec::new();
        let mut compute_queues = Vec::new();
        let mut transfer_queues = Vec::new();

        for queue in queues.into_iter() {
            let family_index = queue.queue_family_index();
            let flags = families_properties[family_index as usize].queue_flags;
            let present_support = device.surface_support(family_index, surface).unwrap();

            if flags.contains(QueueFlags::GRAPHICS) {
                graphics_queues.push((queue, flags, present_support));
            } else if flags.contains(QueueFlags::COMPUTE) {
                compute_queues.push((queue, flags, present_support));
            } else if flags.contains(QueueFlags::TRANSFER) {
                transfer_queues.push((queue, flags, present_support));
            }
        }

        Self {
            graphics_queues,
            compute_queues,
            transfer_queues,
        }
    }

    pub(super) fn get(&self, queue_type: QueueType) -> Result<Arc<Queue>, &str> {
        match queue_type {
            QueueType::GraphicsPresent => self.graphics_present(),
            QueueType::Graphics => self.graphics(),
            QueueType::Compute => self.compute(),
            QueueType::Transfer => self.transfer(),
        }
    }

    fn graphics(&self) -> Result<Arc<Queue>, &str> {
        match self.graphics_queues.iter().next() {
            Some((queue, _, _)) => Ok(queue.clone()),
            None => Err("Graphics queue was not found"),
        }
    }

    fn graphics_present(&self) -> Result<Arc<Queue>, &str> {
        match self.graphics_queues.iter().find(|(_, _, present)| *present) {
            Some((queue, _, _)) => Ok(queue.clone()),
            None => Err("Graphics queue supporting presentation was not found"),
        }
    }

    fn compute(&self) -> Result<Arc<Queue>, &str> {
        if let Some((queue, _, _)) = self.compute_queues.iter().next() {
            Ok(queue.clone())
        } else if let Ok(queue) = self.graphics() {
            Ok(queue)
        } else {
            Err("Compute queue was not found")
        }
    }

    fn transfer(&self) -> Result<Arc<Queue>, &str> {
        if let Some((queue, _, _)) = self.transfer_queues.iter().next() {
            Ok(queue.clone())
        } else if let Ok(queue) = self.compute() {
            Ok(queue)
        } else {
            Err("Transfer queue was not found")
        }
    }
}

pub enum QueueType {
    GraphicsPresent,
    Graphics,
    Compute,
    Transfer,
}
