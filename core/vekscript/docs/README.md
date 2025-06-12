# Documentação do VekScript

Este diretório contém a documentação detalhada do VekScript, uma linguagem desenvolvida para descrever animações, curvas e keyframes de forma simples e flexível. O objetivo do VekScript é facilitar a criação, manipulação e exportação de animações para diferentes aplicações 3D e 2D.

## Sobre o VekScript

O VekScript é uma linguagem de domínio específico (DSL) projetada para descrever animações baseadas em keyframes, curvas de interpolação e objetos animáveis. Com uma sintaxe clara e modular, permite que artistas, desenvolvedores e técnicos criem animações complexas de maneira intuitiva.

## Estrutura da Documentação

A documentação está organizada da seguinte forma:

```
core/
└── vekscript/
    ├── README.md             # visão geral da linguagem
    ├── grammar.md            # especificação da gramática .vek
    ├── examples/
    │   ├── animation1.vek
    │   └── complex_curve.vek
    ├── docs/
    │   ├── syntax.md         # explicação de sintaxe e palavras-chave
    │   ├── types.md          # tipos suportados (object, keyframe, etc.)
    │   ├── interpolation.md  # curva bezier, linear, step, etc.
    │   └── roadmap.md        # o que vai entrar futuramente
```

### Descrição dos Arquivos

- **README.md** (este arquivo): Visão geral do VekScript e da documentação.
- **grammar.md**: Especificação formal da gramática da linguagem `.vek`, útil para implementadores e para entender a estrutura dos arquivos.
- **examples/**: Exemplos práticos de arquivos `.vek` demonstrando animações simples e complexas.
- **docs/syntax.md**: Explicação detalhada da sintaxe da linguagem e das palavras-chave disponíveis.
- **docs/types.md**: Descrição dos tipos suportados pelo VekScript, como `object`, `keyframe`, entre outros.
- **docs/interpolation.md**: Detalhamento dos métodos de interpolação suportados (Bezier, linear, step, etc.) e como utilizá-los.
- **docs/roadmap.md**: Planejamento e funcionalidades previstas para versões futuras do VekScript.

---

Para mais detalhes sobre cada aspecto da linguagem, consulte os arquivos correspondentes nesta pasta.
