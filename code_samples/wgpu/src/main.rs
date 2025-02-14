use wgpu::util::DeviceExt;

// Vamos trabalhar com matrizes 2x2, ou seja, 4 elementos do tipo f32.
const MATRIX_SIZE: usize = 4;
const BUFFER_SIZE: u64 = (std::mem::size_of::<f32>() * MATRIX_SIZE) as u64;

// Função async para executar o código principal
async fn run() {
    // Cria a instância do WGPU com os backends primários (Vulkan, Metal, DX12, etc.)
    let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);

    // Solicita um adapter (dispositivo) compatível com a GPU
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        })
        .await
        .expect("Falha ao encontrar um adapter");

    // Exibe informações sobre o adapter utilizado
    let adapter_info = adapter.get_info();
    println!("Usando adapter: {:?}", adapter_info);

    // Solicita o dispositivo e a fila (queue)
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None,
        )
        .await
        .expect("Falha ao criar o dispositivo");

    // Definindo as duas matrizes (vetores de 4 f32 em ordem row-major)
    // Matriz A:
    // [1, 2]
    // [3, 4]
    let matrix_a: [f32; MATRIX_SIZE] = [1.0, 2.0, 3.0, 4.0];

    // Matriz B:
    // [5, 6]
    // [7, 8]
    let matrix_b: [f32; MATRIX_SIZE] = [5.0, 6.0, 7.0, 8.0];

    // Cria buffers para A e B (read-only) e para C (resultado, que será escrito pela GPU)
    let buffer_a = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Buffer A"),
        contents: bytemuck::cast_slice(&matrix_a),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let buffer_b = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Buffer B"),
        contents: bytemuck::cast_slice(&matrix_b),
        usage: wgpu::BufferUsages::STORAGE,
    });

    // Buffer para a matriz resultado C; inicializado com zeros
    let buffer_c = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Buffer C"),
        size: BUFFER_SIZE,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    // Shader em WGSL para multiplicação de matrizes 2x2.
    // Cada thread (workgroup) calcula um elemento da matriz resultado.
    let shader_source = r#"
        // Declaração dos buffers:
        @group(0) @binding(0)
        var<storage, read> matrix_a: array<f32>;
        @group(0) @binding(1)
        var<storage, read> matrix_b: array<f32>;
        @group(0) @binding(2)
        var<storage, read_write> matrix_c: array<f32>;

        // Função compute: cada thread usa sua posição (i, j) para calcular:
        // C[i][j] = A[i][0]*B[0][j] + A[i][1]*B[1][j]
        @compute @workgroup_size(1, 1, 1)
        fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
            // global_id.x -> linha (i), global_id.y -> coluna (j)
            let i: u32 = global_id.x;
            let j: u32 = global_id.y;

            // Como a matriz é 2x2, o número de colunas é 2.
            var sum: f32 = 0.0;
            for (var k: u32 = 0u; k < 2u; k = k + 1u) {
                // Índice para A: i*2 + k
                // Índice para B: k*2 + j
                sum = sum + matrix_a[i * 2u + k] * matrix_b[k * 2u + j];
            }
            // Salva o resultado em C na posição (i,j)
            matrix_c[i * 2u + j] = sum;
        }
    "#;

    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Matrix Multiplication Shader"),
        source: wgpu::ShaderSource::Wgsl(shader_source.into()),
    });

    // Cria o layout do bind group com três entradas (A, B, C)
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Bind Group Layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 2,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: false },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Bind Group"),
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer_a.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: buffer_b.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: buffer_c.as_entire_binding(),
            },
        ],
    });

    // Cria o pipeline layout e o pipeline compute
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: Some(&pipeline_layout),
        module: &shader_module,
        entry_point: "main",
    });

    // Cria um encoder para os comandos
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Command Encoder"),
    });

    {
        // Inicia a passagem de compute
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
        });
        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        // Despacha um grid de 2x2 workgroups (um para cada elemento da matriz resultado)
        compute_pass.dispatch_workgroups(2, 2, 1);
    }

    // Cria um buffer de leitura para copiar o resultado da matriz C
    let readback_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Readback Buffer"),
        size: BUFFER_SIZE,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    // Copia o conteúdo do buffer C para o buffer de leitura
    encoder.copy_buffer_to_buffer(&buffer_c, 0, &readback_buffer, 0, BUFFER_SIZE);

    // Submete os comandos para a GPU
    queue.submit(Some(encoder.finish()));

    // Mapeia o buffer de leitura para acessar os dados na CPU
    let buffer_slice = readback_buffer.slice(..);
    buffer_slice.map_async(wgpu::MapMode::Read, |_| {});
    device.poll(wgpu::Maintain::Wait);
    let data = buffer_slice.get_mapped_range();
    // Converte os bytes lidos para um slice de f32
    let result: &[f32] = bytemuck::cast_slice(&data);
    println!("Resultado da multiplicação (matriz C):");
    println!("[[{}, {}],", result[0], result[1]);
    println!(" [{}, {}]]", result[2], result[3]);

    // Libera o mapeamento do buffer
    drop(data);
    readback_buffer.unmap();
}

fn main() {
    pollster::block_on(run());
}
