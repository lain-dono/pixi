use winit::dpi::PhysicalSize;

pub struct SwapChain {
    pub swap_chain: wgpu::SwapChain,
    pub surface: wgpu::Surface,

    pub present_mode: wgpu::PresentMode,
    pub format: wgpu::TextureFormat,
    pub size: PhysicalSize<u32>,
    pub scale_factor: f64,
}

impl SwapChain {
    pub fn new(
        device: &wgpu::Device,
        surface: wgpu::Surface,
        size: PhysicalSize<u32>,
        scale_factor: f64,
        format: wgpu::TextureFormat,
        present_mode: wgpu::PresentMode,
    ) -> Self {
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Self {
            swap_chain,
            surface,

            present_mode,
            format,
            size,
            scale_factor,
        }
    }

    pub fn resize(&mut self, device: &wgpu::Device, size: PhysicalSize<u32>) {
        self.size = size;

        let desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: self.format,
            width: size.width,
            height: size.height,
            present_mode: self.present_mode,
        };

        self.swap_chain = device.create_swap_chain(&self.surface, &desc);
    }

    pub fn next_frame(&mut self) -> wgpu::SwapChainOutput {
        self.swap_chain
            .get_next_texture()
            .expect("Timeout when acquiring next swap chain texture")
    }
}
