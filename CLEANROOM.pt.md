# Proveniência de ECOGRAMA e procedimento de implementação independente

[English](CLEANROOM.md) · [Română](CLEANROOM.ro.md) · [Français](CLEANROOM.fr.md) · [Italiano](CLEANROOM.it.md) · [Español](CLEANROOM.es.md) · **Português** · [Deutsch](CLEANROOM.de.md) · [Magyar](CLEANROOM.hu.md)

ECOGRAMA está **implementado de forma independente a partir de especificações públicas, normas e manuais. Não foi copiado nenhum código-fonte de terceiros.** Este ficheiro diz o que foi feito; é uma disciplina de engenharia que mantém a proveniência auditável, não aconselhamento jurídico nem uma garantia estrita do tipo clean-room (não existe uma equipa de especificação separada).

## Regras
1. **Fontes permitidas:** normas e especificações públicas, manuais e artigos revistos por pares, documentação pública e comportamento *visível para o utilizador* de outras ferramentas.
2. **Fontes proibidas:** código descompilado ou divulgado, SDK restritos por EULA, engenharia inversa de formatos proprietários, tabelas geradas, nomes, textos de interface ou conteúdo de exemplo de outras ferramentas.
3. **Proveniência por módulo:** indica a fonte pública no comentário de documentação do módulo e acrescenta uma linha à tabela abaixo.
4. **Dependências:** apenas licenças permissivas (MIT, Apache-2.0, BSD, ISC, Zlib, CC0); `cargo deny check` aplica a lista de `deny.toml`.
5. **Nomes e esquemas próprios** para tudo o que não tenha norma pública.
6. **Projetos relacionados:** listados em `docs/LANDSCAPE.md`; ler a sua *documentação* para delimitar o âmbito é aceitável, copiar o seu código não. Verifica a licença de um projeto antes mesmo de ler o seu código-fonte, se o puderes reimplementar.

## Declaração de assistência de IA
O esqueleto inicial e o seu módulo de arranque foram escritos com um modelo de IA (Claude, Anthropic). Os seus dados de treino não estão sob o controlo do autor; o código foi escrito a partir das fontes públicas indicadas abaixo. Um revisor que precise de maior garantia deve tratá-lo como uma implementação independente com um risco residual documentado.

## Registo de proveniência
| Módulo | Base (pública) | Notas |
|---|---|---|
| hann, dft_magnitude | DSP de manual (janela de Hann, definição da DFT) |  |
