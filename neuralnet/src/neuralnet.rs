pub struct NeuralNet {
    device: wgpu::Device,
    queue: wgpu::Queue,
    encoder: wgpu::CommandEncoder,
    batches: Vec<Vec<Vec<f32>>>,
    layers: Vec<i32>, // vec.length() is the number of layers, i32 is the amount of neurons
    outputs: Vec<Vec<f32>>,
    n_batches: u32,
}

impl NeuralNet {
    pub async fn new(
        inputs: &mut Vec<Vec<f32>>, 
        layers: &mut Vec<i32>, 
        outputs: &mut Vec<f32>, 
        n_batches: u32
    ) -> NeuralNet {
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
        // seperate the inputs into batches
        let batches: Vec<Vec<Vec<f32>>> = inputs.chunks(n_batches as usize).map(|s| s.into()).collect();
    
        // flattening it so its sendable
        let current_batch = batches[0].iter().flatten().copied().collect::<Vec<f32>>();
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
    
        // weights_v.;
        let weights: &[u8] = bytemuck::cast_slice(&weights);
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
    
        let cost_v = [];
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
            compute_pass.dispatch_workgroups(n_batches, 1, 1);
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
    
        Self {
            device,
            queue,
            encoder,
            batches,
            layers,
            outputs,
            n_batches,
        }
    }

    pub async fn train() {
        for batch in batches
    }
    
    async fn compute() {
        // icl killing compute_pass instead of compute_pass.end() is so funny xD
        {
            let mut compute_pass = encoder.begin_compute_pass(&Default::default());
    
            compute_pass.set_pipeline(&cs_pipeline);
            compute_pass.set_bind_group(0, &bind_group, &[]);
            compute_pass.dispatch_workgroups(n_batches, 1, 1);
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
}

