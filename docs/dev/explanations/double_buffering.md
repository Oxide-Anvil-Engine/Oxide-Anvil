# Double Buffering, Triple Buffering e Estratégias de Frame Data

## Resumo
Double buffering e triple buffering são técnicas fundamentais para garantir que CPU e GPU trabalhem de forma eficiente e sem bloqueios, melhorando a fluidez, reduzindo tearing e evitando stalls no pipeline de renderização.

## Double Buffering
- **Conceito:** Mantém dois conjuntos de dados de frame: enquanto a GPU consome um (renderizando/apresentando), a CPU prepara o próximo.
- **Vantagens:**
  - Reduz risco de sobrescrever dados ainda em uso pela GPU.
  - Minimiza stalls: CPU não precisa esperar GPU terminar para começar novo frame.
  - Simples de implementar: alterna ponteiros/índices a cada frame.
- **Desvantagens:**
  - Se a GPU atrasar (frame drop), a CPU pode ficar ociosa esperando liberar o buffer.
  - Pode causar tearing se o swapchain não sincronizar com o vsync.
- **Uso típico:**
  - Jogos 2D/3D simples, aplicações onde latência é mais importante que throughput máximo.

## Triple Buffering
- **Conceito:** Adiciona um terceiro buffer ao ciclo, permitindo que CPU e GPU tenham ainda mais liberdade para trabalhar em paralelo.
- **Vantagens:**
  - Permite que CPU continue preparando frames mesmo se GPU estiver atrasada.
  - Reduz ainda mais o risco de stalls e aumenta o throughput em cenários de alta carga.
  - Ajuda a manter frame pacing estável em pipelines complexos.
- **Desvantagens:**
  - Consome mais memória (um buffer extra para cada recurso transitório).
  - Pode aumentar a latência total (frame pode demorar mais para ser apresentado).
- **Uso típico:**
  - Jogos 3D modernos, engines que priorizam throughput e estabilidade de frame pacing.

## Estratégias de Buffering em Oxide-Anvil
- **Frame Data:**
  - Estruturas de dados (transforms, constantes, comandos) duplicadas ou triplicadas conforme o número de frames em voo.
  - Alternância por índice de frame (`frame_index % num_buffers`).
- **Staging Buffers:**
  - Buffers mapeados persistentes, divididos em regiões (ring buffer) para uploads de dados sem map/unmap frequente.
  - Permite uploads agregados e evita sobrescrita de dados ainda em uso.
- **Swapchain Present Modes:**
  - FIFO (vsync): garante ordem e evita tearing, mas pode limitar throughput.
  - Mailbox: permite múltiplos frames em fila, reduz latência, mas pode variar pacing.
  - Immediate: apresenta assim que pronto, máximo throughput, risco de tearing.

## Considerações Avançadas
- **Sincronização:**
  - Uso de fences/timeline para liberar buffers apenas quando GPU terminar.
  - Minimizar barreiras e agrupar transições para evitar overhead.
- **Reciclagem de recursos:**
  - Pools de buffers transitórios para depth, shadow, intermediários; reciclar por frame.
- **Debug:**
  - Instrumentar tempo de vida dos buffers, medir stalls e latência real entre CPU/GPU.

## Referências
- [Graphics Programming: Double and Triple Buffering](https://learnopengl.com/Advanced-OpenGL/Advanced-GLSL)
- [wgpu Swapchain Present Modes](https://docs.rs/wgpu/latest/wgpu/enum.PresentMode.html)
- [Vulkan Synchronization](https://developer.nvidia.com/blog/vulkan-dos-and-donts/)
