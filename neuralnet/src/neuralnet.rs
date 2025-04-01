use rand::prelude::*;
use rand_distr::Normal;
use indoc::formatdoc;
use pollster::FutureExt;
use std::collections::HashMap;
use crate::utils;

pub struct NeuralNet {
    device: wgpu::Device,
    queue: wgpu::Queue,
    batches: Vec<Vec<Vec<f32>>>,           // for these two, the inner vec is the vector inputs, or vec9f,
    expected_outputs: Vec<Vec<Vec<f32>>>, // for middle vec, its a single batch, and the outer vec groups the batches
    layers: Vec<i32>, // vec.length() is the number of layers, i32 is the amount of neurons
    weights: Vec<Vec<Vec<f32>>>, // for this, we can consider the outer vec as the layers and the two inner as a matrix
    biases: Vec<Vec<f32>>, // outer vec is layers, inner vec is the vector of biases :/
    n_batches: u32,
}


impl NeuralNet {
    pub fn new(
        inputs: &mut Vec<Vec<f32>>, 
        outputs: &mut Vec<Vec<f32>>, 
        layers: &[i32], 
    ) -> Result<NeuralNet, String> {
        pollster::block_on(
            NeuralNet::_new(
                inputs,
                outputs,
                layers.into_iter().cloned().collect(),
                64u32,
            )
        )
    }

    // more direct approach to create NeuralNet
    pub async fn _new(
        inputs: &mut Vec<Vec<f32>>, 
        outputs: &mut Vec<Vec<f32>>, 
        layers: Vec<i32>, 
        n_batches: u32,
    ) -> Result<NeuralNet, String> { 
        // for debugging
        env_logger::init();       
        
        // ~~~ checks to ensure variables are valid ~~~
        // ensure the length of all inputs are the same
        let n_inputs: usize;
        if inputs.is_empty() {
            return Err("there's nothing to train on?...".to_string());
        } else {
            n_inputs = inputs[0].len();
            for input in inputs.iter() {
                if input.len() != n_inputs {
                    return Err("n_inputs must stay consistent".to_string());
                }
            }
        }
 
        // ensure the length of all expected outputs are the same
        let n_outputs: usize;
        if layers.is_empty() {
            return Err("uhh- i mean its possible but like- it would js result in returning inputs...".to_string());
        } else {
            n_outputs = layers[layers.len() - 1usize] as usize;
            for output in outputs.iter() {
                if output.len() != n_outputs {
                    return Err("last layer neurons must be the same as neurons or expected outputs len must be same".to_string());
                }
            }    
        }
        
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

        // layer variables
        let mut rng = rand::rng();

        let mut weights: Vec<Vec<Vec<f32>>> = Vec::new();
        let mut biases: Vec<Vec<f32>> = Vec::new();
        
        let mut n_prev_outputs = n_inputs as i32;
        for n_neurons in layers.iter() {
            weights.push((0..*n_neurons)
                .map(|_| (0..n_prev_outputs)
                .map(|_| {
                    Normal::new(0.0, (2.0 / n_prev_outputs as f32).sqrt())
                        .unwrap().sample(&mut rng)
                }).collect()
            ).collect::<Vec<Vec<f32>>>());

            biases.push(vec![0.01; *n_neurons as usize]);

            n_prev_outputs = *n_neurons;
        }

        Ok(Self {
            device,
            queue,
            batches,
            expected_outputs,
            layers,
            weights,
            biases,
            n_batches,
        })
    }

    pub fn train(&mut self, _learning_rate: f32) {
        let mut current_batch: Vec<f32> = self.batches[0].iter().flatten().copied().collect::<Vec<f32>>();
        let batch: &[u8] = bytemuck::cast_slice(&current_batch);
    
        let batch_buf = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("batch buffer"),
            size: batch.len() as u64,
            usage: 
                wgpu::BufferUsages::STORAGE 
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let mut weights_v: Vec<f32> = self.weights.iter().cloned().into_iter().flatten().flatten().collect::<Vec<f32>>();
        let weights: &[u8] = bytemuck::cast_slice(&weights_v);
        let weights_buf = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("weights buffer"),
            size: weights.len() as u64,
            usage: 
                wgpu::BufferUsages::STORAGE 
                | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        
        let mut biases_v: Vec<f32> = self.biases.iter().cloned().flatten().collect::<Vec<f32>>();
        let biases: &[u8] = bytemuck::cast_slice(&biases_v);
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

        let costs_v: Vec<f32> = vec![0.0; self.n_batches as usize];
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
            label: Some("cost staging buffer one"),
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

        // dynamic code generation
        let n_layers = self.layers.len();
        let n_inputs = self.batches[0][0].len();
        let mut i_weights = String::new();
        let mut i_biases = String::new();
        let mut forward = String::new();
        let mut layers = String::new();

        let mut x_parameter = "X[id.x]".to_string();
        let mut prev_outputs = n_inputs as i32;
        for (i, n_neurons) in self.layers.iter().enumerate() {
            i_weights += &format!("weights{i}: array<array<f32, {prev_outputs}>, {n_neurons}>,\n");
            i_biases += &format!("biases{i}: array<f32, {n_neurons}>,\n");

            if i > 0 {
                x_parameter = format!("al{}", i - 1);
            }
            
            let relu: &str;
            if i == n_layers {
                relu = "al[i] = ReLU(al[i]);";
            } else {
                relu = "";
            }

            forward += &formatdoc!{"
                let al{i} = layer{i}({x_parameter}, weights.weights{i}, biases.biases{i});
            "};

            layers += &formatdoc!{"
                fn layer{i}(
                    X_i: array<f32, {prev_outputs}>,
                    weights_i: array<array<f32, {prev_outputs}>, {n_neurons}>,
                    biases_i: array<f32, {n_neurons}>,
                ) -> array<f32, {n_neurons}> {{
                    var al = array<f32, {n_neurons}>();
                    for (var i = 0; i < {n_neurons}; i += 1) {{
                        for (var j = 0; j < {prev_outputs}; j += 1) {{
                            al[i] += weights_i[i][j] * X_i[j];
                        }}
                        al[i] += biases_i[i];
                        {relu}
                    }}

                    return al;
                }}  

            "};
            
            prev_outputs = *n_neurons;
        }
            
        let cs_module = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("forward propagation module"),
            source: wgpu::ShaderSource::Wgsl(
                utils::template_wgsl(include_str!("neuralnet.wgsl").into(), HashMap::from([
                    ("n_batches".to_string(), self.n_batches.to_string()),
                    ("n_inputs".to_string(), n_inputs.to_string()),
                    ("n_outputs".to_string(), self.layers[n_layers - 1].to_string()),
                    ("n_al".to_string(), (n_layers - 1).to_string()),
                    ("i_weights".to_string(), i_weights),
                    ("i_biases".to_string(), i_biases),
                    ("forward".to_string(), forward),
                    ("layers".to_string(), layers),
                ])).into()
            ),
        });
         
        let cs_pipeline = self.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("compute pipeline"),
            layout: Some(&cs_pipeline_layout),
            module: &cs_module,
            entry_point: Some("forward_pass"),
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
        
        
        self.queue.write_buffer(&batch_buf, 0, batch);
        self.queue.write_buffer(&expected_outputs_buf, 0, expected_outputs);
        self.queue.write_buffer(&costs_buf, 0, costs);

        let mut rng = rand::rng();

        let mut best_average_cost: f32 = 20.0;
        let mut best_weights = weights_v.clone();
        let mut best_biases = biases_v.clone();
        println!("inputs: {:?}\noutputs: {:?}", self.batches[0], self.expected_outputs[0]);

        for a in 0..100000 {
            weights_v = weights_v.iter().map(|w| w + rng.random_range(-3.00..3.00)).collect::<Vec<f32>>();
            let weights = bytemuck::cast_slice(&weights_v);
            biases_v = biases_v.iter().map(|b| b + rng.random_range(-3.00..3.00)).collect::<Vec<f32>>();
            let biases = bytemuck::cast_slice(&biases_v);

            self.queue.write_buffer(&weights_buf, 0, weights);
            self.queue.write_buffer(&biases_buf, 0, biases);
            
            // this is done this way because the variables are previously required for bytelength
            let mut average_cost = self
                .compute(&cs_pipeline, &bind_group, &costs_buf, &costs_staging_buf, &costs_len)
                .block_on();

            for i in 1..self.batches.len() - 2 {
                current_batch = self.batches[i].iter().flatten().copied().collect();
                let batch: &[u8] = bytemuck::cast_slice(&current_batch);

                self.queue.write_buffer(&batch_buf, 0, batch);
                
                average_cost += self
                    .compute(&cs_pipeline, &bind_group, &costs_buf, &costs_staging_buf, &costs_len)
                    .block_on();
            }
            
            average_cost /= (self.batches.len() - 1) as f32;
            if average_cost < best_average_cost {
                best_weights = weights_v.clone();
                best_biases = biases_v.clone();
                best_average_cost = average_cost;
                println!("cost: {}, iteration: {a}", best_average_cost);
            } else {
                weights_v = best_weights.clone();
                biases_v = best_biases.clone();
            }
        }
    }
    
    async fn compute(
        &mut self, 
        cs_pipeline: &wgpu::ComputePipeline, 
        bind_group: &wgpu::BindGroup,
        costs_buf: &wgpu::Buffer,           // these two are output buffers
        costs_staging_buf: &wgpu::Buffer,   // this one ofc stages
        costs_len: &u64,
    ) -> f32 {
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
    
        let costs_buf_slice = costs_staging_buf.slice(..);
        // i hate async istg
        let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
        costs_buf_slice.map_async(wgpu::MapMode::Read, move |cost| {
            sender.send(cost).unwrap()
        });
    
        self.device.poll(wgpu::Maintain::Wait);
    
        // like srsly- i have to copy this from compute shaders 101
        let average_cost: f32;
        if let Some(Ok(())) = receiver.receive().await {
            {
                let costs_raw = &*costs_buf_slice.get_mapped_range();
                let costs: &[f32] = bytemuck::cast_slice(costs_raw);
                let costs_sum: f32 = costs.iter().sum();
                average_cost = costs_sum / costs.len() as f32; 
            }

            costs_staging_buf.unmap();
        } else {
            panic!("uhm");
        }

        average_cost
    }
}
