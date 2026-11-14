# ECOGRAMA — Editor de espectrogramas e restauração de áudio: selecionar no espectrograma, reparar, exportar.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · **Português** · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versão: 0.1.0-alpha.1 · Última atualização: 2026-11-14

🌐 https://ecograma.chiuta.ro

## Estado

Esqueleto inicial (pré-alfa): o workspace compila e os testes passam no CI. Cada funcionalidade prevista é uma tarefa aberta em `docs/issues/`.

## Âmbito previsto

- STFT com janela e salto configuráveis, mapas de cor (viridis, magma, escala de cinzentos) e leitura/escrita WAV (PCM 16/24 bits, vírgula flutuante de 32 bits).
- Uma vista de espectrograma em egui com zoom e deslocamento.
- Seleções retangulares e de laço que produzem uma máscara tempo–frequência, atenuada com ressíntese overlap-add.
- Reparação espectral por interpolação dos quadros vizinhos e remoção do zumbido, que deteta 50/60 Hz e harmónicos.
- Um invólucro de plug-in CLAP (VST3 mais tarde) e uma decisão sobre a relação com `cathar`: depender, interoperar ou contribuir a montante.

Nada disto está ainda implementado: cada ponto é uma tarefa aberta em `docs/issues/`.

## Compilar e testar

```bash
cargo test --workspace
cargo run -p ecograma-cli -- selftest 64 8
```

## Contribuir

Leia `CONTRIBUTING.md`, escolha uma tarefa com a etiqueta `good first issue` em `docs/issues/` e abra um pull request. Nenhuma contribuição é demasiado pequena: uma linha de tabela, um teste ou uma tradução contam.

## Implementação independente

Escrita a partir de especificações públicas e manuais; não foi copiado código-fonte de terceiros (ver `CLEANROOM.md`).

## Licença

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Apoio

Trade-free: sem rastreamento nem publicidade. Se lhe for útil, pode apoiar o autor: https://buymeacoffee.com/echo.of.the.strings
