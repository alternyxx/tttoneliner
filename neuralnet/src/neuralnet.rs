use std::collections::HashMap;
use env_logger;

pub async fn neuralnet(dataset: &mut HashMap<Vec<i8>, i8>) {
    env_logger::init();
    
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });

    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: None,
        force_fallback_adapter: false,
    }).await.unwrap();

    let (device, queue) = adapter.request_device(
        &Default::default(),
        None,
    ).await.unwrap();
    
    // ~~~ Compute shader Inputs ~~~ //
    let x: &[f32] = &[1.0, 2.0, 3.0];
    let x_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("X buffer"),
        size: x.len() as u64,
        usage: 
            wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC
            | wgpu::BufferUsages::STORAGE,
        mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("bind group layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage {
                         read_only: false 
                    },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });
    
    // ~~~ Compute Pipeline ~~~ //
    let cs_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("compute pipeline layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let cs_module = device.create_shader_module(wgpu::include_wgsl!("neuralnet.wgsl"));
    
    let cs_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("compute pipeline"),
        layout: Some(&cs_pipeline_layout),
        module: &cs_module,
        entry_point: Some("cs_main"),
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        cache: None,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("bind group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: x_buf.as_entire_binding(),
            }
        ]
    });

    let mut encoder = device.create_command_encoder(&Default::default());

    // icl killing compute_pass instead of compute_pass.end() is so funny xD
    {
        let mut compute_pass = encoder.begin_compute_pass(&Default::default());

        compute_pass.set_pipeline(&cs_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(1, 1, 1);
    }
    
    queue.submit(Some(encoder.finish()));
}

