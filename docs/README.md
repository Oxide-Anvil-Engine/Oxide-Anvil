Oxide-Anvil é uma engine 2D/3D em Rust com foco em performance e controle total. Hoje usa wgpu + winit como base gráfica e mantém o núcleo desacoplado para permitir backends mais especializados (ex.: Vulkan, áudio de baixa latência, ray tracing futuro).

## Documentação principal
- Arquitetura e organização: [docs/architecture/overview.md](architecture/overview.md)
- Pipeline gráfico e otimizações: [docs/dev/architecture/graphics_pipeline.md](dev/architecture/graphics_pipeline.md)
- Comparativos de engines: [docs/comparisons/engines.md](comparisons/engines.md)
- Tutoriais (índice): [docs/tutorials/README.md](tutorials/README.md)
- Ideias (linguagem/blueprints): [docs/dev/ideas/scripting_blueprints.md](dev/ideas/scripting_blueprints.md)

## Estado atual (curto)
- Suporte inicial: wgpu + winit (desktop). Desenvolvimento primário em Linux, alvo principal Windows.
- Camadas separadas: `core` (contratos), `backend` (implementações), `application` (orquestração).
- Próximos passos: consolidar pipeline gráfico, definir contrato de áudio, preparar guias passo a passo.
