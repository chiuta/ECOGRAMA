# ECOGRAMA — primeras diez tareas

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · **Español** · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Créalas en GitHub con `scripts/create-issues.sh OWNER/REPO`. Comenta en una issue para reclamarla.

1. **STFT con ventana y salto configurables; prueba contra `dft_magnitude`** — `good first issue`, `area:dsp`
   Usa `rustfft` tras una funcionalidad; la DFT ingenua sigue siendo el oráculo. Diferencia máxima por bin < 1e-3.

2. **Mapas de color como datos: viridis, magma, escala de grises** — `good first issue`, `data`, `area:gui`
   Tablas de 256 entradas en `data/*.csv`, un tipo `ColorMap` y una prueba de que cada tabla es monótona en luminancia.

3. **Lectura y escritura de WAV (PCM de 16/24 bits y float de 32 bits)** — `good first issue`, `area:io`
   Implementa o elige un crate de licencia permisiva; pruebas de ida y vuelta; rechaza cabeceras mal formadas sin entrar en pánico.

4. **Vista de espectrograma en egui: renderizado, zoom y desplazamiento** — `help wanted`, `area:gui`
   Opción de frecuencia logarítmica; maneja archivos de 10 minutos mediante teselado. Captura de pantalla en el PR.

5. **Selecciones: rectángulo y lazo producen una máscara tiempo–frecuencia** — `help wanted`, `area:gui`
   La máscara es una estructura dispersa con bordes suaves; prueba la unión y la diferencia de máscaras.

6. **Atenúa la selección con resíntesis overlap-add** — `help wanted`, `area:dsp`
   Prueba de ida y vuelta: sin selección = idéntico al bit dentro de 1e-6; la selección elimina un tono sintético en ≥ 40 dB.

7. **Reparación espectral: rellena una selección interpolando los fotogramas vecinos** — `help wanted`, `area:dsp`
   Primero clics y cortes breves. Prueba con una senoide + un clic; informa del SNR antes/después.

8. **De-hum: detecta 50/60 Hz y armónicos, aplica filtros notch estrechos** — `good first issue`, `area:dsp`
   Prueba con zumbido sintético + ruido; deja intacta la banda del habla (mídela).

9. **Envoltorio de plug-in CLAP (VST3 más adelante)** — `help wanted`, `area:plugin`
   Comprueba primero las licencias del framework de plug-in y de los bindings de VST3; escribe el resultado en `docs/`.

10. **Compara con `cathar` y decide: depender, interoperar o contribuir al proyecto original** — `help wanted`, `docs`
   Mide de-click y reparación espectral con las mismas señales sintéticas; escribe `docs/ADR-001.md`.
