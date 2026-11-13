# ECOGRAMA — Editor de espectrogramas y restauración de audio: seleccionar en el espectrograma, reparar, exportar.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · **Español** · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versión: 0.1.0-alpha.1 · Última actualización: 13.11.2026

## Estado

Esqueleto inicial (pre-alfa): el workspace compila y las pruebas pasan en CI. Cada función prevista es una tarea abierta en `docs/issues/`.

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
