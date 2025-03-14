use std::collections::HashMap;

pub async fn neuralnet(dataset: &mut HashMap<Vec<i8>, i8>) {
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
    
    // funny story i have with this if someone ever reads
    // in my alternyxx.com repo theres this like bindgroup layout
    // i was so confused as to why we dont put this immediatly above bind_group
    // turns out this was also used in pipeline layout and it was included in the commend heading
    // in the alternyxx.com repo and i was lmfao, past me predicted it
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
            }
        ]
    });
    
    let cs_module = device.create_shader_module(wgpu::include_wgsl!("neuralnet.wgsl"));
    let cs_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("compute pipeline"),
        layout: None,
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
                resource: None,
            }
        ]
    });

    let mut encoder = device.create_command_encoder(&Default::default());

    // icl killing compute_pass instead of compute_pass.end() is so funny xD
    {
        let mut compute_pass = encoder.begin_compute_pass(&Default::default());

        compute_pass.set_pipeline(&cs_pipeline);
        compute_pass.dispatch_workgroups(1, 1, 1);
    }
    
    queue.submit(Some(encoder.finish()));
}

