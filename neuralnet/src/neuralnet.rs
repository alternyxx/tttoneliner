use rand::prelude::*;
use pollster::FutureExt;

#[allow(dead_code)] // temporarily 
// ion wanna deal w/ lifetimes
pub struct NeuralNet {
    device: wgpu::Device,
    queue: wgpu::Queue,
    batches: Vec<Vec<Vec<f32>>>,          // for these two, the inner vec is the vector inputs, or vec9f,
    expected_outputs: Vec<Vec<Vec<f32>>>, // for middle vec, its a single batch, and the outer vec groups the batches
    layers: Vec<i32>, // vec.length() is the number of layers, i32 is the amount of neurons
    n_batches: u32,
}

impl NeuralNet {
    pub async fn new(
        inputs: &mut Vec<Vec<f32>>, 
        outputs: &mut Vec<Vec<f32>>, 
        layers: Vec<i32>, 
        n_batches: u32,
    ) -> NeuralNet {        
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
        
        // seperate the inputs into batches
        let batches: Vec<Vec<Vec<f32>>> = inputs.chunks(n_batches as usize).map(|s| s.into()).collect();
        let expected_outputs: Vec<Vec<Vec<f32>>> = outputs.chunks(n_batches as usize).map(|s| s.into()).collect();

        Self {
            device,
            queue,
            batches,
            expected_outputs,
            layers,
            n_batches,
        }
    }

    // this function is created because i want js/ts template literals and
    // pipeline constants aren't enough
    fn template_wgsl(wgsl: &str) {
        let mut templating = false;
        let mut template_variable: String = "".to_owned();

        for (_i, char) in wgsl.chars().enumerate() {
            if char == '?' {
                templating = true;
            }

            if templating {
                template_variable += &char.to_string();
            }
        }
    }

    pub fn train(&self) {
        // flattening it so its sendable
        let current_batch: Vec<f32> = self.batches[4].iter().flatten().copied().collect::<Vec<f32>>();
        let batch: &[u8] = bytemuck::cast_slice(&current_batch);
    
        let batch_buf = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("batch buffer"),
            size: batch.len() as u64,
            usage: 
                wgpu::BufferUsages::STORAGE 
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        

        // temporarily hard coded
        let mut rng = rand::rng();

        let weights_v: Vec<f32> = (0..9)
            .map(|_| (0..9).map(|_| rng.random_range(-10.0..10.0)).collect())
            .collect::<Vec<Vec<f32>>>()
            .iter().flatten().copied().collect::<Vec<f32>>();

        let weights: &[u8] = bytemuck::cast_slice(&weights_v);
        let weights_buf = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("weights buffer"),
            size: weights.len() as u64,
            usage: 
                wgpu::BufferUsages::STORAGE 
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
    
        
        let biases_v: &[f32] = &[2.0; 9];
        let biases: &[u8] = bytemuck::cast_slice(biases_v);
        let biases_buf = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("weights buffer"),
            size: biases.len() as u64,
            usage: 
                wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        
        let current_expected_outputs: Vec<f32> = self.expected_outputs[4].iter().flatten().copied().collect::<Vec<f32>>();
        let expected_outputs: &[u8] = bytemuck::cast_slice(&current_expected_outputs);
        let expected_outputs_buf = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("expected outputs buffer"),
            size: expected_outputs.len() as u64,
            usage:
                wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        
        let costs_v: Vec<f32> = vec![vec![0.0f32; 9]; 64]
            .iter().flatten().copied().collect::<Vec<f32>>();
        let costs: &[u8] = bytemuck::cast_slice(&costs_v);
        let costs_len = costs.len() as u64; // we'll be doing a lot of computes so might as well

        let costs_buf = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("cost buffer"),
            size: costs_len,
            usage: 
                wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        
        let costs_staging_buf = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("cost staging buffer"),
            size: costs_len,
            usage: 
                wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        
        // Bind group layout // 
        let bind_group_layout = self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { 
                            read_only: true 
                        },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }, wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { 
                            read_only: true 
                        },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }, wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { 
                            read_only: true 
                        },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }, wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { 
                            read_only: true 
                        },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }, wgpu::BindGroupLayoutEntry {
                    binding: 4,
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
        let cs_pipeline_layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("compute pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
    
        let cs_module = self.device.create_shader_module(wgpu::include_wgsl!("neuralnet.wgsl"));
         
        let cs_pipeline = self.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("compute pipeline"),
            layout: Some(&cs_pipeline_layout),
            module: &cs_module,
            entry_point: Some("cs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            cache: None,
        });
    
        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
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
                    resource: expected_outputs_buf.as_entire_binding(),
                }, wgpu::BindGroupEntry {
                    binding: 4,
                    resource: costs_buf.as_entire_binding(),
                }, 
            ]
        });
        
        // we're temporarily just sending a single batch
        self.queue.write_buffer(&batch_buf, 0, batch);
        self.queue.write_buffer(&weights_buf, 0, weights);
        self.queue.write_buffer(&biases_buf, 0, biases);
        self.queue.write_buffer(&expected_outputs_buf, 0, expected_outputs);
        self.queue.write_buffer(&costs_buf, 0, costs);

        self.compute(&cs_pipeline, &bind_group, &costs_buf, &costs_staging_buf, &costs_len).block_on();
    }
    
    async fn compute(
        &self, 
        cs_pipeline: &wgpu::ComputePipeline, 
        bind_group: &wgpu::BindGroup,
        costs_buf: &wgpu::Buffer,           // these two are output buffers
        costs_staging_buf: &wgpu::Buffer,   // this one ofc stages
        costs_len: &u64,
    ) {
        let mut encoder = self.device.create_command_encoder(&Default::default());
        
        // icl killing compute_pass instead of compute_pass.end() is so funny xD
        {
            let mut compute_pass = encoder.begin_compute_pass(&Default::default());
    
            compute_pass.set_pipeline(cs_pipeline);
            compute_pass.set_bind_group(0, bind_group, &[]);
            compute_pass.dispatch_workgroups(self.n_batches, 1, 1);
        }
        
        encoder.copy_buffer_to_buffer(&costs_buf, 0, &costs_staging_buf, 0, *costs_len);
    
        self.queue.submit(Some(encoder.finish()));
    
        let cost_buf_slice = costs_staging_buf.slice(..);
        // i hate async istg
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        cost_buf_slice.map_async(wgpu::MapMode::Read, move |cost| {
            sender.send(cost).unwrap()
        });
    
        self.device.poll(wgpu::Maintain::Wait);
    
        // like srsly- i have to copy this from compute shaders 101
        if let Some(Ok(())) = receiver.receive().await {
            let data_raw = &*cost_buf_slice.get_mapped_range();
            let data: &[f32] = bytemuck::cast_slice(data_raw);
            println!("{:?}", data);
        }
    }
}
