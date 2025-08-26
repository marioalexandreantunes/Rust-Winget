# winget_update

Pequeno utilitário para Windows escrito em Rust que automatiza a verificação e instalação de atualizações usando o winget. O programa:

- Executa `winget upgrade` para obter a tabela de pacotes com atualização disponível.
- Faz o parse da saída para extrair Nome, Id, versão atual e versão disponível.
- Mostra a lista encontrada e, para cada pacote não excluído, executa `winget update -q <ID> --accept-package-agreements --accept-source-agreements`.
- Ao final, executa `rustup update` para atualizar o toolchain do Rust.
- Imprime um resumo do que foi atualizado e do que foi ignorado.

A aplicação exibe um pequeno logo em ASCII e usa cores no terminal para facilitar a leitura.

## Como funciona

1. Logo na inicialização, o programa verifica se o code page do console é UTF-8 (CP 65001) e apenas informa caso não seja.
2. Obtém a lista de upgrades com `winget upgrade` e faz o parse das linhas da tabela.
3. Exibe os pacotes encontrados (nome e versões).
4. Para cada pacote:
   - Se o nome contiver qualquer item da lista de exclusões (array `excluded_apps`), o pacote é pulado.
   - Caso contrário, roda `winget update` usando o ID do pacote (mais confiável que o nome) com aceite automático dos termos.
5. Ao final, chama `rustup update` e imprime um resumo com contagem de atualizados e ignorados.

Observações de parsing:
- O código ignora as duas primeiras linhas da saída tabular e interrompe a leitura quando encontra a linha de rodapé que contém "atualizações disponíveis" (pt-BR) ou "upgrades available" (en), ou uma linha em branco.
- O nome do aplicativo pode conter espaços; o parser reagrupa os campos para extrair corretamente `name`, `id`, `current_version`, `available_version` e `source`.

## Requisitos

- Windows 10/11 (winget só está disponível no Windows).
- winget instalado e disponível no PATH.
- Para compilar a partir do código-fonte: Rust (cargo) instalado.
- Para que a etapa final funcione: `rustup` instalado (senão a chamada a `rustup update` falhará).

## Dependências (Cargo.toml)

- Runtime:
  - `colored` — cores no terminal.
  - `figlet-rs` — geração do logo em ASCII.
  - `winapi` (feature `consoleapi`) — consulta ao code page do console.
- Build (Windows):
  - `winres` — inclui recursos do Windows (ícone) no executável.
- Nota: `dirs` está listado, mas não é usado no código atual e pode ser removido se desejado.

## Build e execução

A partir da raiz do repositório:

- Compilar em release (recomendado):

  bash
  cargo build --release

- Executar (via cargo ou o binário gerado):

  bash
  cargo run --release

O executável de release ficará em `target/release/`.

## Uso

- Execute o binário. Será exibida a lista de pacotes com atualização disponível e, em seguida, o processo de atualização.
- Pode ser necessário executar com privilégios elevados (Administrador) para que algumas instalações sejam concluídas pelo winget.

## Configuração

- Lista de exclusões: definida em `src/main.rs` no array `excluded_apps`.
  - Padrão: `["AutoIt", "Discord"]`.
  - A verificação é por substring no nome do pacote (se o nome contém o texto). Ajuste conforme sua necessidade e recompile.
- Aceite de termos: o programa chama `winget update` com `--accept-package-agreements` e `--accept-source-agreements`.

## Recursos do Windows (ícone)

- Em Windows, `build.rs` usa `winres` para embutir o ícone `ICON.ico` no executável. Certifique-se de manter o arquivo `ICON.ico` na raiz do projeto para que o ícone seja aplicado.

## Limitações e melhorias possíveis

- A análise da saída depende do formato tabular do `winget upgrade` e pode quebrar se a formatação mudar em versões futuras do winget.
- Atualmente para na linha de resumo (em português/inglês). Se usar outro idioma, ajuste as palavras-chave no código.
- Possíveis melhorias:
  - Tornar a lista de exclusões configurável por CLI/arquivo/env (ex.: com `clap`).
  - Adicionar modo "dry-run" (simular sem aplicar) e logs em arquivo.
  - Tornar o parser mais resiliente ou usar um formato estruturado se/quando disponível.

## Metadados do projeto

- Nome: `winget_update`
- Versão: 0.1.0
- Perfil de release: otimizações para tamanho, LTO, `panic = "abort"`, símbolos de debug removidos (ver `Cargo.toml`).

