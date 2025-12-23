# Comparativo de engines

Foco: controle, performance e caminho de extensão técnica para quem prefere código e previsibilidade.

## Resumo rápido
| Engine        | Linguagem/base      | Controle/perf | Extensão de backend/render | Ferramentas/fluxo | Observações |
|---------------|---------------------|---------------|----------------------------|-------------------|-------------|
| Godot         | GDScript/C#         | Baixo-médio   | Baixa (render fechado)     | Editor forte      | Ótima para começar; scripting limita perf e acesso baixo nível |
| Bevy          | Rust (ECS)          | Médio         | Média (modular)            | Sem editor nativo | Boa base em Rust; menos foco em backends customizados |
| Unity         | C#                  | Médio         | Média (SRP/URP/HDRP)       | Editor maduro     | Fluxo artístico forte; overhead e menor previsibilidade |
| Unreal        | C++/Blueprint       | Alto          | Alta (custom render)       | Editor pesado     | Poder AAA, custo e curva altos |
| Oxide-Anvil   | Rust + HAL          | Alto          | Alta (HAL troca backends)  | Editor Rust (fut.)| Prioriza previsibilidade; curva técnica maior; editor planejado (nativo/Web) |

## Síntese (quando escolher)
- Godot: protótipos 2D/3D simples, foco artístico, time pequeno, pouca exigência de perf.
- Bevy: quer Rust hoje, aceita ECS padrão, não precisa custom backend imediato.
- Unity: fluxo artístico e produção acelerada, aceita overhead e scripting C#.
- Unreal: AAA, custom render pesado, equipe com C++ e tempo para lidar com complexidade.
- Oxide-Anvil: controle profundo, backends sob medida (wgpu hoje, Vulkan/RT futuro), previsibilidade de latência e editor em Rust previsto.

## Detalhe técnico avançado por engine

### Godot
- Render: pipeline fechado; customizações profundas exigem forks; perf em 3D limitada por GDscript e overhead do editor.
- Linguagens: GDScript simples porém com GC e baixa performance; C# ajuda mas continua distante do metal; C++ modules demandam build complexa.
- Ferramentas: editor excelente, import de assets simples; bom para arte; profiling razoável; pouco controle de pipeline.
- Perf: CPU-bound em scripts; GC pode introduzir picos; batching automático ajuda em 2D; 3D sofre em cenas pesadas.

### Bevy
- Render: renderer próprio sobre wgpu; modular; troca de backend não é foco principal, mas arquitetura em Rust facilita forks.
- Linguagem: Rust ECS com system scheduling; segurança e paralelismo bem suportados; curva de ECS + borrow checker.
- Ferramentas: sem editor visual nativo; hot-reload de assets/texturas; profiling via bevy_mods/comunidade.
- Perf: boa base CPU devido a Rust; overhead de ECS e agendamento é previsível; GPU depende de wgpu (limites atuais para RT avançado).

### Unity
- Render: URP/HDRP (SRP) permitem shader graph e algum controle, mas não são tão abertos quanto um backend próprio; C# limita hot-path.
- Linguagem: C# com GC; determinismo/latência sofrem em picos; Burst/Jobs ajudam mas exigem disciplina.
- Ferramentas: editor e asset pipeline muito fortes; ótimo para artistas; profiling integrado; boa documentação.
- Perf: boa para médio porte; picos de GC e abstrações podem afetar jogos CPU-bound; RT disponível mas com overhead e tuning complexo.

### Unreal
- Render: altamente extensível em C++; suporte maduro a RT, virtual shadow maps, lumen (iluminação global) e Nanite.
- Linguagem: C++ para hot-path, Blueprints para iteração; é possível mover código crítico para C++; exige build/config pesada.
- Ferramentas: editor completo, profiling avançado, asset pipeline robusto; custo em tamanho e complexidade.
- Perf: alto teto; demanda equipe experiente; tuning fino necessário para latência e memória.

### Oxide-Anvil
- Render: HAL separando contratos e backends; wgpu hoje; caminho claro para backend Vulkan para RT e controle de memória/pipelines; foco em previsibilidade.
- Linguagem: Rust; sem GC; determinismo mais fácil; curva de aprendizado maior; APIs priorizam hot-path enxuto.
- Ferramentas: editor planejado em Rust (nativo/Web) para cena, materiais, pipelines e inspeção de perf; HUD de perf planejado.
- Perf: busca latência previsível e throughput alto; possibilidade de backends sob medida (áudio dedicado, Vulkan, WebGPU seletivo).

## Comparação técnica aprofundada
- Linguagem/runtime: GC (Godot GDScript, Unity C#) tende a picos; C++/Rust (Unreal, Oxide-Anvil, Bevy) permitem previsibilidade e inline de hot-path.
- Extensibilidade de pipeline: Unreal e Oxide-Anvil (via HAL/backends) têm mais espaço para custom render; Unity oferece SRP mas com barreiras; Godot é mais fechado; Bevy é modular mas preso ao wgpu atual.
- Ferramentas/editor: Unity/Unreal lideram em fluxo artístico; Godot é leve e acessível; Bevy/Oxide-Anvil priorizam código e perf; Oxide planeja editor em Rust para fechar a lacuna sem abrir mão de controle.
- Ray tracing: Unreal maduro; Unity disponível com overhead; Godot experimental e limitado; wgpu limitado; Oxide-Anvil planeja RT via backend Vulkan.
- Asset/import: Unity/Unreal fortes; Godot simples; Bevy community-driven; Oxide-Anvil dependerá de pipeline custom (a definir conforme assets do projeto).
- Multiplataforma: Unity/Unreal amplas; Godot boa em desktop/mobile; Bevy/Oxide-Anvil dependem de wgpu/targets; WebGPU e editor Web são opções condicionadas à previsibilidade.

## Performance e pipeline (qualitativo)
- CPU: C++/Rust permitem linhas de execução e dados enxutos; C# e GDScript adicionam overhead/GC; ECS em Rust (Bevy/Oxide) dá paralelismo mais seguro.
- GPU: renderers fechados (Godot) limitam tuning; SRP (Unity) dá algum controle; Unreal permite tuning profundo; Oxide-Anvil quer controle via backend e HAL; Bevy segue wgpu.
- RT: Unreal > Unity > wgpu (limitado) > Godot (experimental); Oxide-Anvil só pleno com backend Vulkan futuro.
- Hot-path: Godot e Unity dependem de otimizações internas; Bevy/Oxide permitem microtuning em Rust; Unreal permite tuning em C++ mas é pesado.

## Uso recomendado (reforçado)
- Prototipagem visual/art-heavy: Godot/Unity.
- Controle técnico em Rust com ECS pronta: Bevy.
- AAA ou custom render pesado com equipe grande: Unreal.
- Controle profundo em Rust, backends sob medida e latência previsível: Oxide-Anvil.
