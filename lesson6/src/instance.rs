

pub struct Instance {
    position: cgmath::Vector3<f32>,
    pub(crate) rotation: cgmath::Quaternion<f32>,
}

impl Instance {
    pub fn new(position: cgmath::Vector3<f32>, rotation: cgmath::Quaternion<f32>) -> Self {
        Self {
            position, rotation
        }
    }

    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            model: (cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation)).into(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    model: [[f32; 4]; 4],
}

impl InstanceRaw {
    pub(crate) fn description<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            // Переключаем режим работы с VertexStepMode::Vertex на VertexStepMode::Instance
            // Это значит, что шейдер будет итерироваться по инстансам, а не векторам
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                // mat4 имеет размер 4 векторов vec4, поэтому я делаю 4 слота для каждого вектора
                wgpu::VertexAttribute {
                    offset: 0,
                    // Пока что в вершинном шейдере я использую 0 и 1 location.
                    // 2, 3 и 4 зарезервирую на будущее, поэтому здесь shader_location = 5
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}