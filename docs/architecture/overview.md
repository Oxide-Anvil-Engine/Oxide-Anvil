# Visão geral da arquitetura

## Resumo rápido
- Engine 2D/3D em Rust com foco em controle e performance previsível.
- Núcleo desacoplado: `core` (contratos), `backend` (implementações), `application` (orquestração e loop).
- Base atual: wgpu + winit; alvo principal Windows; dev em Linux. Planejado: backend Vulkan para RT/controle fino e backends de áudio dedicados.
- Editor futuro: escrito em Rust, pensado para rodar nativo e, possivelmente, via Web (wasm) para colaboração/portabilidade.
- Documentação: visão geral e comparativos em `docs/`; materiais de otimização e ideias em `docs/dev/`.

## Estrutura de pastas (alto nível)
```
Oxide-Anvil/
├─ crates/
│  ├─ application/   # Entrada que orquestra engine e integra backends
│  ├─ backend/       # Implementações HAL específicas (ex.: wgpu, futuro Vulkan)
│  └─ core/          # Contratos/interfaces (traits) e tipos básicos
├─ docs/             # Documentação estável (arquitetura, comparativos, visão geral)
├─ docs/dev/         # Materiais avançados (pipeline, otimizações, ideias)
├─ examples/         # Exemplos de uso da engine
├─ target/           # Saída de build (ignorada no git)
├─ Cargo.lock
└─ Cargo.toml        # Workspace
```

## Camadas (detalhado)
- `core`: contratos para renderização, áudio, entrada, assets, ciclo de vida; define swapchain/command submission e tipos fundamentais.
- `backend`: implementa os contratos para um alvo (wgpu hoje). Planejado: Vulkan (RT, controle de memória/pipelines), backends de áudio de baixa latência.
- `application`: loop principal (`winit`), integração de backends, cenas/sistemas, telemetria e seleção dinâmica de backend quando aplicável.

## Roadmap e backends
- WGPU (atual): estabilizar, otimizar upload/batching, medir frame pacing.
- Vulkan (planejado): destravar RT completo, controle fino de pipelines e memória.
- Áudio dedicado (planejado): WASAPI/ALSA/Pulse com foco em latência e depuração.
- Web (possível): editor e runtime via wasm/webgpu se mantiver previsibilidade.

## Editor (futuro)
- Implementação em Rust para manter performance e compartilhamento de código com a engine.
- Modo nativo e possibilidade de front-end Web (wasm) para colaboração e uso leve.
- Deve expor: cena, materiais, pipelines configuráveis, inspeção de perf, e integração com DSL/blueprints (ver ideias).

## Documentação e tutoriais
- `docs/`: visão geral, arquitetura, comparativos e guias estáveis.
- `docs/comparisons/`: comparação detalhada de engines.
- `docs/tutorials/`: índice e tutoriais passo a passo (eventos → frame, passes gráficos, backends, áudio, DSL, perf, build/deploy).
- `docs/dev/`: pipeline gráfico (detalhe/otimização), ideias/DSL/blueprints e material avançado.

## Público-alvo
Desenvolvedores que querem controle, previsibilidade e abertura para backends sob medida, aceitando a curva de Rust e de arquitetura para obter consistência de performance.
