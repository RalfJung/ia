use crate::Renderer;
use libremexre::errors::Result;
use std::sync::Arc;
use vulkano::{
    command_buffer::CommandBuffer,
    image::SwapchainImage,
    swapchain::{acquire_next_image, AcquireError},
    sync::{now, FlushError, GpuFuture},
};
use winit::Window;

impl Renderer {
    /// Draws the command buffer created on the given image.
    pub fn draw<F, T>(&mut self, cb: F) -> Result<()>
    where
        F: FnOnce(Arc<SwapchainImage<Window>>) -> T,
        T: 'static + CommandBuffer,
    {
        let (image_num, acquire_future) = loop {
            match acquire_next_image(self.swapchain.clone(), None) {
                Ok(r) => break r,
                Err(AcquireError::OutOfDate) => {
                    self.recreate_swapchain()?;
                }
                Err(err) => return Err(Box::new(err)),
            }
        };

        let command_buffer = cb(self.images[image_num].clone());

        let r = self
            .cleanup_future
            .take()
            .unwrap()
            .join(acquire_future)
            .then_execute(self.queue.clone(), command_buffer)?
            .then_swapchain_present(self.queue.clone(), self.swapchain.clone(), image_num)
            .then_signal_fence_and_flush();
        match r {
            Ok(future) => {
                self.cleanup_future = Some(Box::new(future));
                Ok(())
            }
            Err(FlushError::OutOfDate) => {
                self.cleanup_future = Some(Box::new(now(self.device.clone())));
                self.recreate_swapchain()
            }
            Err(e) => {
                self.cleanup_future = Some(Box::new(now(self.device.clone())));
                Err(Box::new(e))
            }
        }
    }
}
