use std::sync::{
    mpsc::{Receiver, Sender},
    Arc,
};

use vulkano::instance::InstanceExtensions;
use winit::window::Window;

use crate::{renderer::RendererIncomingEvent, window::ApplicationEvent};

pub struct WindowMessenger {
    pub initial_receiver: Receiver<(Arc<Window>, InstanceExtensions)>,
    pub run_receiver: Receiver<RendererIncomingEvent>,
}
pub struct RendererMessenger {
    pub initial_sender: Sender<(Arc<Window>, InstanceExtensions)>,
    pub run_sender: Sender<RendererIncomingEvent>,
}
