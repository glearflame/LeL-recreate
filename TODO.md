# TODO — Glear-Mine / horizon-recreate

Plano de correções e melhorias do projeto. Itens marcados com `[x]` estão concluídos.

## 🔴 Segunda — Dia de Correções Críticas (Launcher) ✅ TODAS CORRIGIDAS

### C1 — Race condition: múltiplas instalações concorrentes ✅ CORRIGIDO
**Arquivo:** `crates/horizon-launcher/src/main.rs`  
**Status:** Corrigido em commit `a8865fd`  
**Solução:** Flag `installing: bool` impede múltiplas chamadas simultâneas a `start_install`.

### C2 — Mutex envenenado → panic da UI ✅ CORRIGIDO
**Arquivo:** `crates/horizon-launcher/src/main.rs`  
**Status:** Corrigido em commit `a8865fd`  
**Solução:** Todos os `lock().unwrap()` trocados por `lock().unwrap_or_else(|e| e.into_inner())`.

### C3 — Temp ZIP não removido em erro ✅ CORRIGIDO
**Arquivo:** `crates/horizon-launcher/src/main.rs`  
**Status:** Corrigido em commit `a8865fd`  
**Solução:** Função `cleanup_temp_zip()` chamada em todos os caminhos de `Err` e no sucesso.

### C4 — Caminho de instalação relativo ✅ CORRIGIDO
**Arquivo:** `crates/horizon-launcher/src/main.rs`  
**Status:** Corrigido em commit `a8865fd`  
**Solução:** Caminho absoluto via `dirs::data_dir()` → `~/.local/share/Glear-Mine/versions/26.3`.

### M1 — Sem progresso real durante extração do ZIP ✅ CORRIGIDO
**Arquivo:** `crates/horizon-launcher/src/main.rs`  
**Status:** Corrigido em commit `a8865fd`  
**Solução:** `InstallState::Extracting { progress }` atualizado dentro do loop de extração (0.75 → 1.0).

### M2 — Campo de código de convite não digita nada ✅ CORRIGIDO
**Arquivo:** `crates/horizon-launcher/src/main.rs`  
**Status:** Corrigido em commit `a8865fd`  
**Solução:** Campo `invite_code: String` no `LauncherApp`, usado em `ui.text_edit_singleline(&mut app.invite_code)`.

### M3 — Starvation na fila do hub quando lobby cheio ✅ CORRIGIDO
**Arquivo:** `crates/horizon-hub/src/lib.rs`  
**Status:** Corrigido em commit `a8865fd`  
**Solução:** Removido `break` no `process_queue` quando lobby está cheio; agora pula e continua a fila.

### M4 — `remove_dir_all` falha se `versions/26.3` for um arquivo ✅ CORRIGIDO
**Arquivo:** `crates/horizon-launcher/src/main.rs`  
**Status:** Corrigido em commit `a8865fd`  
**Solução:** Verifica `is_file()`/`is_dir()` antes de remover; usa `remove_file` para arquivos.

### W1 — Chaves duplicadas no workspace Cargo.toml
**Arquivo:** `Cargo.toml` raiz  
**Solução:** Remover chaves redundantes de `[workspace]` (mantém apenas `[workspace.package]`).

### W2 — URL do PixelDrain hardcoded
**Arquivo:** `crates/horizon-launcher/src/main.rs`  
**Solução:** Tornar a URL configurável (variável de ambiente ou arquivo de config).

### W3 — `CARGO_MANIFEST_DIR` fixo para assets ✅ CORRIGIDO
**Arquivo:** `crates/horizon-client/src/lib.rs`  
**Status:** Corrigido em commit `8e1931d`  
**Solução:** `assets_dir()` usa `std::env::current_exe()` para resolver a pasta assets relativa ao binário.

---

## 🟡 Terça — Integração Launcher ↔ Game (pendente)

- [ ] Adicionar argumentos CLI ao `horizon-client`: `--server`, `--version`, `--offline`, `--render-distance` (usar `clap`)
- [ ] Botão "Jogar" no launcher invoca `horizon-client` via `std::process::Command`
- [ ] Botão "Jogar no Hub (Glear-Mine)" conecta em `127.0.0.1:25565`
- [ ] Botão "Jogar em Servidor..." abre dialog para IP personalizado

---

## 🟢 Quarta — Estrutura de Pastas e Reorganização (pendente)

- [ ] Criar nova estrutura de pastas (separar `game`, `server`, `data`, etc.)
- [ ] Mover código existente sem quebrar compilação
- [ ] Criar `horizon-data` crate (Registry genérico, loader JSON)
- [ ] Separar `StubRenderer` do client (feature flag em `horizon-render`)

---

## 🔵 Quinta — Servidor Dedicado e Multiplayer (pendente)

- [ ] Criar `horizon-server` crate: loop de tick, `TcpListener`, conexão por jogador
- [ ] Packet framing (length-prefixed), codec versionado em `protocol::codec`
- [ ] Teste: client conecta no server, recebe chunks, anda

---

## 🟣 Sexta — Render Backend Real + Polish (pendente)

- [ ] Adicionar feature flag `wgpu`/`vulkan` em `horizon-render`
- [ ] Cliente usa backend real via feature flag (Stub continua default)
- [ ] Documentação: `ARCHITECTURE.md`, `CONTRIBUTING.md`, diagramas Mermaid

---

## Como executar cada item

```bash
cd /tmp/opencode/horizon-recreate

# Build
cargo build --release

# Testes
cargo test --release

# Rodar launcher
cargo run --release -p horizon-launcher

# Rodar hub
cargo run --release -p horizon-hub

# Rodar cliente
cargo run --release -p horizon-client

# Commit e push
git add -A
git commit -m "feat: descricao da tarefa"
git push origin main
```

---

## Estado atual do repositório

- **URL:** https://github.com/glearflame/horizon-recreate
- **Branch:** `main`
- **Último commit:** `a8865fd` — todas as correções críticas e médias aplicadas
- **Compila:** ✅ `cargo build --release` OK
- **Testes:** ✅ 22/22 passam

---

**Nota:** Este é um plano de execução manual. Cada item acima precisa ser executado por você ao chamar a sessão novamente.