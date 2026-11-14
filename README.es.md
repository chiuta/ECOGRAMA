# ECOGRAMA — Editor de espectrogramas y restauración de audio: seleccionar en el espectrograma, reparar, exportar.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · **Español** · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versión: 0.1.0-alpha.1 · Última actualización: 2026-11-14

🌐 https://ecograma.chiuta.ro

## Estado

Esqueleto inicial (pre-alfa): el workspace compila y las pruebas pasan en CI. Cada función prevista es una tarea abierta en `docs/issues/`.

## Alcance previsto

- STFT con ventana y salto configurables, mapas de color (viridis, magma, escala de grises) y lectura/escritura WAV (PCM 16/24 bits, flotante de 32 bits).
- Una vista de espectrograma en egui con zoom y desplazamiento.
- Selecciones rectangulares y de lazo que producen una máscara tiempo–frecuencia, atenuada con resíntesis overlap-add.
- Reparación espectral por interpolación de los fotogramas vecinos y eliminación del zumbido, que detecta 50/60 Hz y armónicos.
- Un envoltorio de plug-in CLAP (VST3 más adelante) y una decisión sobre la relación con `cathar`: depender, interoperar o contribuir aguas arriba.

Nada de esto está implementado todavía: cada punto es una tarea abierta en `docs/issues/`.

## Compilar y probar

```bash
cargo test --workspace
cargo run -p ecograma-cli -- selftest 64 8
```

## Contribuir

Lee `CONTRIBUTING.md`, elige una tarea con la etiqueta `good first issue` en `docs/issues/` y abre un pull request. Ninguna contribución es demasiado pequeña: una fila de tabla, una prueba o una traducción cuentan.

## Implementación independiente

Escrita a partir de especificaciones públicas y manuales; no se ha copiado código fuente de terceros (véase `CLEANROOM.md`).

## Licencia

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Apoyo

Trade-free: sin rastreo ni publicidad. Si te resulta útil, puedes apoyar al autor: https://buymeacoffee.com/echo.of.the.strings
