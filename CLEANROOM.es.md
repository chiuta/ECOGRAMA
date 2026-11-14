# Procedencia de ECOGRAMA y procedimiento de implementación independiente

[English](CLEANROOM.md) · [Română](CLEANROOM.ro.md) · [Français](CLEANROOM.fr.md) · [Italiano](CLEANROOM.it.md) · **Español** · [Português](CLEANROOM.pt.md) · [Deutsch](CLEANROOM.de.md) · [Magyar](CLEANROOM.hu.md)

ECOGRAMA está **implementado de forma independiente a partir de especificaciones públicas, normas y manuales. No se ha copiado ningún código fuente de terceros.** Este archivo dice qué se hizo; es una disciplina de ingeniería que mantiene la procedencia auditable, no asesoramiento legal ni una garantía estricta de tipo clean-room (no existe un equipo de especificación separado).

## Reglas
1. **Fuentes permitidas:** normas y especificaciones públicas, manuales y artículos revisados por pares, documentación pública y comportamiento *visible para el usuario* de otras herramientas.
2. **Fuentes prohibidas:** código descompilado o filtrado, SDK restringidos por EULA, ingeniería inversa de formatos propietarios, tablas generadas, nombres, textos de interfaz o contenido de ejemplo de otras herramientas.
3. **Procedencia por módulo:** nombra la fuente pública en el comentario de documentación del módulo y añade una línea a la tabla de abajo.
4. **Dependencias:** solo licencias permisivas (MIT, Apache-2.0, BSD, ISC, Zlib, CC0); `cargo deny check` aplica la lista de `deny.toml`.
5. **Nombres y esquemas propios** para todo lo que no tenga un estándar público.
6. **Proyectos relacionados:** se enumeran en `docs/LANDSCAPE.md`; leer su *documentación* para acotar el alcance está bien, copiar su código no. Comprueba la licencia de un proyecto antes incluso de leer su código fuente si podrías reimplementarlo.

## Declaración de asistencia de IA
El esqueleto inicial y su módulo de partida se escribieron con un modelo de IA (Claude, Anthropic). Sus datos de entrenamiento no están bajo el control del autor; el código se escribió a partir de las fuentes públicas citadas abajo. Quien revise y necesite mayor garantía debería tratarlo como una implementación independiente con un riesgo residual documentado.

## Registro de procedencia
| Módulo | Base (pública) | Notas |
|---|---|---|
| hann, dft_magnitude | DSP de manual (ventana de Hann, definición de la DFT) |  |
