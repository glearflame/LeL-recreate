# horizon-recreate

Recriação do zero do núcleo do **Horizon** (partes de 100% de autoria do
[horizon-rust/horizon](https://github.com/horizon-rust/horizon)), escrita em
Rust.

Este projeto **não contém** código nem assets de terceiros:
- ❌ Discord / bot do Discord
- ❌ Site (playhorizon.dev)
- ❌ Páginas de patrocínio / loja
- ❌ Código ou assets da Mojang / Microsoft (Minecraft)

O que é recriado do zero (estrutura de crates inspirada no Horizon):

- `horizon-core` — posições (bloco/chunk/seção), números aleatórios (xoroshiro), geometria de colisão
- `horizon-world` — blocos, estados de bloco, chunks, mundo, registro de blocos, NBT
- `horizon-render` — trait de renderer e implementação stub
- `horizon-protocol` — tipos de pacote de rede
- `horizon-assets` — localização de recursos e carregamento
- `horizon-client` — loop de jogo e executável do cliente
- `horizon-hub` — **hub/lobby Glear-Mine** (salas, filas de conexão, gerenciamento de jogadores)
- `horizon-launcher` — launcher/hub visual **Glear-Mine** (estilo Horizon Offline Launcher: sidebar, abas Início/Mods/Servidores/Guarc/Registro de build, botão "Entrar com a Microsoft" apenas visual)
  - Botão **Instalar**: baixa o `.zip` da versão 26.3 do PixelDrain, descompacta em `versions/26.3/` e libera o botão **Jogar** após a instalação.

## Créditos

Este trabalho é baseado na estrutura e no design público do
[horizon-rust/horizon](https://github.com/horizon-rust/horizon)
(Minecraft: Java Edition reescrito em Rust), distribuído sob a
[licença MIT](https://github.com/horizon-rust/horizon/blob/main/LICENSE).

O módulo `horizon-hud/src/client_render/` contém código derivado de
`crates/horizon-client-render/src` do projeto original **horizon-rust/horizon**
(MIT). Todos os créditos vão para **horizon-rust/horizon** e seus contribuidores.

Partes de terceiros continuam excluídas: Discord, site, patrocínio,
código/assets da Mojang/Microsoft.

## Build

```bash
cargo build --release
cargo run -p horizon-client
cargo run -p horizon-launcher   # hub visual Glear-Mine (com botão Instalar/Jogar 26.3)
cargo run -p glear-mine-hub     # hub de lobby
```

## Licença

MIT. Veja [LICENSE](LICENSE). Dependências mantêm suas próprias licenças.
