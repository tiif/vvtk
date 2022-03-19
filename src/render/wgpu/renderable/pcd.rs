use wgpu::{DepthStencilState, Device, include_wgsl, PipelineLayout, RenderPipeline, TextureFormat, VertexBufferLayout};
use wgpu::CompareFunction::Less;
use crate::formats::PointCloud;
use crate::formats::pointxyzrgba::PointXyzRgba;
use crate::pcd::PointCloudData;
use crate::render::wgpu::antialias::AntiAlias;
use crate::render::wgpu::renderable::Renderable;

impl Renderable for PointCloudData {
    fn buffer_layout_desc<'a>() -> VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: 16,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: 12,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        }
    }

    fn create_render_pipeline(device: &Device, format: TextureFormat, layout: Option<&PipelineLayout>) -> RenderPipeline {
        let shader = device.create_shader_module(&include_wgsl!("../shaders/pcd.wgsl"));

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Self::buffer_layout_desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::PointList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: Some(DepthStencilState {
                depth_write_enabled: true,
                depth_compare: Less,
                stencil: Default::default(),
                format: TextureFormat::Depth32Float,
                bias: Default::default()
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            // If the pipeline will be used with a multiview render pass, this
            // indicates how many array layers the attachments will have.
            multiview: None,
        })
    }

    fn antialias(&self) -> AntiAlias {
        let pcd = self.clone();
        let pointcloud: PointCloud<PointXyzRgba> = pcd.into();
        let first_point = pointcloud.points.get(0).unwrap();
        let mut max_x = first_point.x;
        let mut max_y = first_point.y;
        let mut max_z = first_point.z;
        let mut min_x = first_point.x;
        let mut min_y = first_point.y;
        let mut min_z = first_point.z;

        for point in pointcloud.points {
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
            max_z = max_z.max(point.z);
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            min_z = min_z.min(point.z);
        }
        let mut max = (max_x - min_x).max(max_y - min_y).max(max_z - min_z);
        if max == 0.0 {
            max = 1.0
        }
        AntiAlias {
            x: (max_x - min_x) / 2.0,
            y: (max_y - min_y) / 2.0,
            z: (max_z - min_z) / 2.0,
            scale: max,
        }
    }

    fn bytes(&self) -> &[u8] {
        self.data()
    }

    fn vertices(&self) -> usize {
        self.data().len() / 16
    }

}
