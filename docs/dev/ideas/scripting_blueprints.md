# Linguagem/blueprints (ideia)

Objetivo: fluxo visual ou scripting interpretado em desenvolvimento, com caminho para compilação/integração nativa em Rust em build ou em modos de performance. Sem abrir mão de determinismo e previsibilidade.

## Resumo rápido
- Dev: interpretar ou JIT para iterar rápido; hot-reload; validação estática antes de rodar.
- Build/perf: gerar Rust ou bytecode otimizado; reduzir overhead de runtime; alinhar-se às regras de ownership.
- IR comum: mesma árvore alimenta intérprete e gerador de código; evita duplicação de lógica.
- Integração: bindings diretos com `core` (eventos, sistemas, render/áudio/input) sem GC pesado.

## Modos de execução
- Modo dev (interpretação/JIT): execução imediata, inspeção, depuração rápida; coleta de métricas; menos otimizações de registro/memória.
- Modo perf (build): transpilar para Rust ou gerar bytecode otimizado; remover checks extras; inline de chamadas; alinhar dados a SoA quando viável.

## Arquitetura proposta
- Front-end: DSL textual e, futuramente, editor visual de nós; salva em formato declarativo (ron/toml/json) versionável.
- IR comum: AST simplificada que representa fluxos, eventos, dados e efeitos; usada tanto por intérprete quanto pelo gerador de Rust.
- Intérprete: execução direta do IR para hot-reload; foca em clareza e diagnósticos (locais, tipos, ciclos de vida de handles).
- JIT (opcional): backend cranelift/similar para acelerar hot-reload mantendo flexibilidade; fallback para interpretação pura quando não disponível.
- Gerador Rust: produz módulos/funcões com bindings seguros; FFI mínima; favorece zero-copy e passagem de handles fortes.

## Integração com a engine
- Eventos: mapeamento direto de eventos do `core` (input, ciclo, render callbacks) para handlers da DSL.
- Sistemas: permitir registrar sistemas de jogo/logic que o scheduler do `application` executa; aderir às regras de borrowing para evitar aliasing indevido.
- Render/áudio: expor chamadas controladas (submit de comandos, triggers de áudio) sem permitir acesso irrestrito a dados internos.

## Build pipeline
- Etapas: validar → baixar deps → gerar Rust/bytecode → compilar → empacotar assets.
- Checks: tipos, ciclo de vida, recursos usados; orçamento de CPU/GPU opcional por script.
- Saída: módulos Rust que entram no `cargo build` ou bytecode carregado pelo runtime com verificação de versão/ABI.

## Debug, inspeção e hot-reload
- Hot-reload: recarregar scripts/graphs sem reiniciar; invalidar apenas o necessário; reter estado quando seguro.
- Diagnósticos: mostrar stack trace da DSL, tempos por nó/sistema, alocações e contagem de eventos processados.
- Instrumentação: hooks para logs estruturados, contadores e export para HUD de perf.

## Performance e dados
- Dados alinhados: incentivar SoA para hot-path; evitar cópias; passar handles para buffers compartilhados.
- Orçamentos: limitar custo de cada script (tempo/alloc) em dev para detectar regressões cedo.
- Cache: memorizar funções geradas/JIT; invalidar por hash do IR/asset.

## Riscos/mitigações
- Complexidade da toolchain: começar com DSL textual simples antes do editor visual; aumentar escopo gradualmente.
- Divergência entre DSL e APIs: gerar bindings a partir de descrições únicas no `core` para manter 1:1.
- Segurança/ownership: código gerado deve seguir regras de borrow e tipos; sem ponteiros crus; validação estática antes de aceitar build.
- Portabilidade: garantir que o caminho de JIT tenha fallback para interpretação em plataformas sem suporte.
