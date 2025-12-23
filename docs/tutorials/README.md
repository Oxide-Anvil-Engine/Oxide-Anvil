# Tutoriais (índice)

## Objetivo
Organizar guias passo a passo para uso e extensão da Oxide-Anvil, em PT-BR (fonte). Cada tutorial terá resumo, pré-requisitos, passos detalhados e seção de verificação (build/run/perf).

## Páginas planejadas
- Pipeline de jogo: do evento ao frame apresentado (input → update → render → present).
- Pipeline gráfico na prática: montar passes (shadow, forward/forward+, post) e medir frame pacing.
- Integração de backend gráfico alternativo: roteiro de protótipo Vulkan partindo do HAL.
- Áudio dedicado: integrar backend de baixa latência e validar latência/estabilidade.
- DSL/Blueprints: criar e rodar um script/graph simples, gerar Rust/bytecode no build.
- Checklists de performance: CPU (ECS, dados), GPU (binds, batching, culling, RT), assets (mips/compactação).
- Build e deploy: configurações para Windows alvo, validação em Linux dev, notas sobre Web (quando aplicável).

## Estrutura sugerida por tutorial
- Resumo e objetivo.
- Pré-requisitos (toolchain, OS alvo, assets mínimos).
- Passos numerados com comandos e explicações.
- Seção “Por que” (racional técnico das escolhas).
- Seção de validação: como rodar, o que medir, critérios de sucesso.
- Próximos passos/variações.
