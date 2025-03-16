use std::collections::HashMap;

const BATCHES: u32 = 64;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vec9F {  
    x: [f32; 3], _padding1: f32,  // so apparantly wgsl vec3fs are 16 bytes...
    y: [f32; 3], _padding2: f32,  // ngl- the compiler stuff w/ # in rust are quite weird
    z: [f32; 3], _padding3: f32,  // i like them for #[test] but comon #[repr(C)]?? :sob:
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Mat9x9F {
    a: Vec9F, b: Vec9F, c: Vec9F,
    d: Vec9F, e: Vec9F, f: Vec9F,
    g: Vec9F, h: Vec9F, i: Vec9F,
}

pub async fn neuralnet(dataset: &mut HashMap<i32, i32>) {
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
    // to anyone, messing w/ this later, im running on 2 hours of sleep
    // so anyways, since this obv needs explanation... (n ion wanna change main.rs anymore)
    // we had a board as 111121111 and we make it into 9 f32s like [1.0, 1.0, ...]
    let tmp: Vec<Vec<f32>> = dataset.keys().copied().map(|i| {
        i.to_string().chars().map(|c| c.to_digit(10).unwrap() as f32).collect()
    }).collect(); 
    
    // and seperate them into batches of 64 (workgroup size to access)
    let batches: Vec<Vec<Vec<f32>>> = tmp.chunks(64).map(|s| s.into()).collect();

    let current_batch_v = batches[0].iter().flatten().copied().collect::<Vec<f32>>();
    let current_batch = current_batch_v.chunks(9).map(|board| {
        Vec9F {
            x: [board[0], board[1], board[2]], _padding1: 0.0, 
            y: [board[3], board[4], board[5]], _padding2: 0.0, 
            z: [board[6], board[7], board[8]], _padding3: 0.0,
        }
    }).collect::<Vec<Vec9F>>();

    let batch: &[u8] = bytemuck::cast_slice(&current_batch);

    let batch_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("batch buffer"),
        size: batch.len() as u64,
        usage: 
            wgpu::BufferUsages::UNIFORM 
            | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    queue.write_buffer(&batch_buf, 0, batch);

    let mut weights_v: [f32; 81] = [0.0f32; 81];
    weights_v.fill(1.0f32);
    weights_v.;
    let weights: &[u8] = bytemuck::cast_slice(&weights_v);
    let weights_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("weights buffer"),
        size: weights.len() as u64,
        usage: 
            wgpu::BufferUsages::UNIFORM 
            | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    queue.write_buffer(&weights_buf, 0, weights);

    let biases_v: &[f32] = &[2.0];
    let biases: &[u8] = bytemuck::cast_slice(biases_v);
    let biases_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("weights buffer"),
        size: biases.len() as u64,
        usage: 
            wgpu::BufferUsages::UNIFORM 
            | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    queue.write_buffer(&biases_buf, 0, biases);

    let cost: &[u8] = bytemuck::cast_slice(&[0.0f32]);
    let cost_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("cost buffer"),
        size: cost.len()  as u64,
        usage: 
            wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    queue.write_buffer(&cost_buf, 0, cost);

    let cost_staging_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("cost staging buffer"),
        size: cost.len() as u64,
        usage: 
            wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("bind group layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }, wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }, wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }, wgpu::BindGroupLayoutEntry {
                binding: 3,
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
                resource: batch_buf.as_entire_binding(),
            }, wgpu::BindGroupEntry {
                binding: 1,
                resource: weights_buf.as_entire_binding(),
            }, wgpu::BindGroupEntry {
                binding: 2,
                resource: biases_buf.as_entire_binding(),
            }, wgpu::BindGroupEntry {
                binding: 3,
                resource: cost_buf.as_entire_binding(),
            }, 
        ]
    });

    let mut encoder = device.create_command_encoder(&Default::default());

    // icl killing compute_pass instead of compute_pass.end() is so funny xD
    {
        let mut compute_pass = encoder.begin_compute_pass(&Default::default());

        compute_pass.set_pipeline(&cs_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(BATCHES, 1, 1);
    }
    
    encoder.copy_buffer_to_buffer(&cost_buf, 0, &cost_staging_buf, 0, cost.len() as u64);

    queue.submit(Some(encoder.finish()));

    let cost_buf_slice = cost_staging_buf.slice(..);
    // i hate async istg
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    cost_buf_slice.map_async(wgpu::MapMode::Read, move |cost| {
        sender.send(cost).unwrap()
    });

    device.poll(wgpu::Maintain::Wait);

    // like srsly- i have to copy this from compute shaders 101
    if let Some(Ok(())) = receiver.receive().await {
        let data_raw = &*cost_buf_slice.get_mapped_range();
        let data: &[f32] = bytemuck::cast_slice(data_raw);
        println!("{:?}", data);
    }
}

