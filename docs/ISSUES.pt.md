# ECOGRAMA — primeiras dez tarefas

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · **Português** · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Crie-as no GitHub com `scripts/create-issues.sh OWNER/REPO`. Comente numa issue para a reivindicar.

1. **STFT com janela e salto configuráveis; teste contra `dft_magnitude`** — `good first issue`, `area:dsp`
   Use `rustfft` atrás de uma funcionalidade; a DFT ingênua continua como oráculo. Diferença máxima por bin < 1e-3.

2. **Mapas de cores como dados: viridis, magma, escala de cinza** — `good first issue`, `data`, `area:gui`
   Tabelas de 256 entradas em `data/*.csv`, um tipo `ColorMap` e um teste de que cada tabela é monótona em luminância.

3. **Leitura e escrita de WAV (PCM de 16/24 bits e float de 32 bits)** — `good first issue`, `area:io`
   Implemente ou escolha um crate de licença permissiva; testes de ida e volta; rejeite cabeçalhos malformados sem entrar em pânico.

4. **Vista de espectrograma no egui: renderização, zoom e deslocamento** — `help wanted`, `area:gui`
   Opção de frequência logarítmica; trate arquivos de 10 minutos por mosaico. Captura de tela no PR.

5. **Seleções: retângulo e laço produzem uma máscara tempo–frequência** — `help wanted`, `area:gui`
   A máscara é uma estrutura esparsa com bordas suaves; teste a união e a diferença de máscaras.

6. **Atenue a seleção com ressíntese overlap-add** — `help wanted`, `area:dsp`
   Teste de ida e volta: sem seleção = idêntico ao bit dentro de 1e-6; a seleção remove um tom sintético em ≥ 40 dB.

7. **Reparo espectral: preencha uma seleção interpolando os quadros vizinhos** — `help wanted`, `area:dsp`
   Primeiro cliques e quedas curtas. Teste com uma senoide + um clique; informe o SNR antes/depois.

8. **De-hum: detecte 50/60 Hz e harmônicos, aplique filtros notch estreitos** — `good first issue`, `area:dsp`
   Teste com zumbido sintético + ruído; deixe a banda da fala intacta (meça-a).

9. **Invólucro de plug-in CLAP (VST3 depois)** — `help wanted`, `area:plugin`
   Verifique primeiro as licenças do framework de plug-in e dos bindings de VST3; escreva o resultado em `docs/`.

10. **Compare com `cathar` e decida: depender, interoperar ou contribuir à montante** — `help wanted`, `docs`
   Meça de-click e reparo espectral nos mesmos sinais sintéticos; escreva `docs/ADR-001.md`.
