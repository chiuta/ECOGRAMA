# Contribuir — ECOGRAMA

[English](CONTRIBUTING.md) · [Română](CONTRIBUTING.ro.md) · [Français](CONTRIBUTING.fr.md) · [Italiano](CONTRIBUTING.it.md) · [Español](CONTRIBUTING.es.md) · **Português** · [Deutsch](CONTRIBUTING.de.md) · [Magyar](CONTRIBUTING.hu.md)

Obrigado pela ajuda. ECOGRAMA é um projeto implementado de forma independente: não copies código, recursos nem textos de outras ferramentas; trabalha a partir de especificações públicas, manuais e comportamento observado, e indica a fonte que usaste (vê `CLEANROOM.md`).

1. Escolhe uma tarefa em `docs/issues/` (ou na sua issue no GitHub) e diz num comentário que a assumes. As tarefas com a etiqueta `good first issue` não exigem conhecimento prévio do projeto.
2. Compila e testa: `cargo test --workspace`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`.
3. Acrescenta um teste para cada comportamento que acrescentares ou corrigires. Os testes não podem precisar de acesso à rede nem de ficheiros privados; constrói os dados de exemplo no próprio teste.
4. Os analisadores e tudo o que lê dados não fiáveis nunca devem entrar em pânico: devolve um erro.
5. Acrescenta uma linha à tabela de proveniência em `CLEANROOM.md` por cada módulo novo (que fonte pública segue).
6. Regista as alterações visíveis para o utilizador em `CHANGELOG.md`. A documentação segue o fluxo do README em oito línguas (inglês, romeno, francês, italiano, espanhol, português, alemão, húngaro); uma correção de tradução é uma contribuição bem-vinda.
7. Commits: assunto curto no imperativo, assinados (`git commit -s`, Developer Certificate of Origin). Problemas de segurança: vê `SECURITY.md` (em privado, não numa issue pública).
