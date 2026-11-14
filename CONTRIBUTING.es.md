# Contribuir — ECOGRAMA

[English](CONTRIBUTING.md) · [Română](CONTRIBUTING.ro.md) · [Français](CONTRIBUTING.fr.md) · [Italiano](CONTRIBUTING.it.md) · **Español** · [Português](CONTRIBUTING.pt.md) · [Deutsch](CONTRIBUTING.de.md) · [Magyar](CONTRIBUTING.hu.md)

Gracias por ayudar. ECOGRAMA es un proyecto implementado de forma independiente: no copies código, recursos ni textos de otras herramientas; trabaja a partir de especificaciones públicas, manuales y comportamiento observado, e indica qué fuente has usado (consulta `CLEANROOM.md`).

1. Elige una tarea de `docs/issues/` (o de su issue en GitHub) y di en un comentario que la tomas. Las tareas etiquetadas `good first issue` no requieren conocimientos previos del proyecto.
2. Compila y prueba: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`.
3. Añade una prueba por cada comportamiento que añadas o corrijas. Las pruebas no deben necesitar acceso a la red ni archivos privados; construye los datos de ejemplo en la propia prueba.
4. Los analizadores y todo lo que lea datos no fiables nunca deben entrar en pánico: devuelve un error.
5. Añade una línea a la tabla de procedencia de `CLEANROOM.md` por cada módulo nuevo (qué fuente pública sigue).
6. Anota los cambios visibles para el usuario en `CHANGELOG.md`. La documentación sigue el flujo del README en ocho idiomas (inglés, rumano, francés, italiano, español, portugués, alemán, húngaro); una corrección de traducción es una contribución bienvenida.
7. Commits: asunto breve en imperativo, firmados (`git commit -s`, Developer Certificate of Origin). Problemas de seguridad: consulta `SECURITY.md` (en privado, no en una issue pública).
