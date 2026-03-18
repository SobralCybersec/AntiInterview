<div align="center">

<h1 align="center">
  <img src="https://i.imgur.com/dGls2xX.png" width="50" />
  Anti-Interview
</h1>

Oculte janelas durante sessões de compartilhamento de tela com integração em nível de sistema

</div>

---

<h1 align="center">
  <img src="https://i.imgur.com/dwyUWDH.gif" width="30"/> Funcionalidades
</h1>

* **Ocultação de Janelas**: Oculte janelas específicas das APIs de captura de tela
* **Bandeja do Sistema**: Minimize para a bandeja com ícone nativo
* **Visualização ao Vivo**: Preview em tempo real com captura D3D11
* **Atalhos**: Teclas de atalho personalizáveis
* **Multi-Monitor**: Suporte completo para múltiplos monitores
* **Arquitetura DDD**: Código limpo, manutenível e pronto para produção
* **Abstrações Zero-Cost**: Implementação Rust de alta performance
* **Auto-Atualização**: Atualização automática da lista de janelas

---

<h1 align="center">
  <img src="https://i.imgur.com/eu3StDB.gif" width="30"/> Stack Tecnológica
</h1>

<p align="center">
  <img src="https://go-skill-icons.vercel.app/api/icons?i=rust,windows&size=64" />
</p>

* Rust 1.70+
* Windows API (Win32)
* Captura de Tela D3D11
* Injeção de DLL (dll-syringe)
* Framework egui
* Integração com Bandeja do Sistema

---

<h1 align="center">
  <img src="https://cdn-icons-png.flaticon.com/512/1157/1157109.png" width="30"/> Arquitetura
</h1>

<div align="center">

Construído com princípios de **Domain-Driven Design (DDD)** e arquitetura limpa:

```
Camada de Domínio       → Entidades e regras de negócio
Camada de Aplicação     → Casos de uso e interfaces  
Camada de Infraestrutura → Windows API, D3D11, File I/O, Tray
Camada de Apresentação   → GUI com gerenciamento de estado
```

</div>

---

## O que faz?

Durante entrevistas online ou reuniões, oculte aplicações específicas da captura de tela enquanto continua usando-as normalmente.

<p align="center">
  <img src="./assets/screenshots/local-view.png" width="400" alt="O que você vê" />
  <img src="./assets/screenshots/remote-view.png" width="400" alt="O que eles veem" />
</p>

**Esquerda**: Sua visão local | **Direita**: O que outros veem durante o compartilhamento

Janelas selecionadas são ocultadas das APIs de captura de tela, tornando-as invisíveis para os participantes da reunião enquanto permanecem totalmente funcionais.

**Compatível com**: Zoom, MS Teams, Discord, OBS e qualquer aplicação que use APIs de captura de tela do Windows.

---

## Detalhes Técnicos

### Tecnologia Principal

Usa injeção de DLL para chamar [SetWindowDisplayAffinity](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowdisplayaffinity) com flag `WDA_EXCLUDEFROMCAPTURE`.

### Performance

- Abstrações zero-cost
- Alocações mínimas de memória
- Iteradores eficientes
- Buffers pré-alocados
- Captura D3D11 otimizada

### Qualidade de Código

- Sem `unwrap()` ou `expect()` em produção
- Tratamento adequado de erros com `Result`
- Princípio da Responsabilidade Única
- Inversão de Dependência
- Código auto-documentado

---

## Instalação

### Guia Rápido

Veja o [Guia Rápido de Configuração](./QUICK_START.md) para instruções passo a passo.

### Requisitos

- Windows 10 v2004 ou superior
- Privilégios de administrador para injeção de DLL

### Download

Baixe a versão mais recente na página de [Releases](https://github.com/yourusername/anti-interview/releases).

### Aviso de Antivírus

Este software pode acionar avisos de antivírus devido às técnicas de injeção de DLL. Isso é um falso positivo e pode ser ignorado com segurança.

---

## Uso

1. Execute `anti-interview.exe`
2. Selecione as janelas para ocultar da lista
3. Inicie sua sessão de compartilhamento de tela
4. Janelas ocultas não aparecerão na tela compartilhada
5. Minimize para a bandeja do sistema para operação em segundo plano

### Atalhos

| Atalho | Ação |
|--------|------|
| `Ctrl+Shift+S` | Screenshot |
| `Ctrl+Shift+H` | Ocultar janela selecionada |
| `Ctrl+Shift+I` | Mostrar/Ocultar GUI |

### Bandeja do Sistema

- Clique no botão **X** → Minimiza para a bandeja
- Clique com botão direito no ícone da bandeja → Mostrar menu
- Clique em **Mostrar** → Restaurar janela
- Clique em **Sair** → Fechar aplicação

---

## Configuração

Arquivo de configuração: `%APPDATA%\AntiInterview\config.toml`

```toml
[hotkeys]
screenshot = "Ctrl+Shift+S"
hide_window = "Ctrl+Shift+H"
show_gui = "Ctrl+Shift+I"

[ui]
dark_theme = true
show_preview = false
hide_from_taskbar = false
window_size = [320.0, 540.0]

[behavior]
auto_refresh = true
refresh_interval_ms = 1000
minimize_to_tray = true
```

---

## Compilando do Código Fonte

### Pré-requisitos

- Rust 1.70 ou superior
- Windows SDK

### Passos de Compilação

```bash
git clone https://github.com/yourusername/anti-interview.git
cd anti-interview
cargo build --release
```

O executável estará em `target/release/anti-interview.exe`

### Executar Testes

```bash
cargo test --workspace
```


---

<h1 align="center">
  <img src="https://i.imgur.com/O7HwCZt.gif" width="30"/> Roadmap
</h1>

* [x] Ocultação de janelas com injeção de DLL
* [x] Integração com bandeja do sistema
* [x] Preview de captura de tela D3D11
* [x] Suporte multi-monitor
* [x] Refatoração com arquitetura DDD
* [x] Gerenciamento de configuração
* [x] Banner animado com GIF
* [ ] UI de customização de atalhos
* [ ] Perfis de janelas
* [ ] Regras de auto-ocultação
* [ ] Iniciar com o Windows

---

<h1 align="center"><img src="https://i.imgur.com/6nSJzZ2.gif" width="35"/> Referências</h1>

<h2 align="center">
  
**Windows API**: [SetWindowDisplayAffinity](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowdisplayaffinity)  <img src="https://go-skill-icons.vercel.app/api/icons?i=windows&size=32" width="40" />

</h2>

<h2 align="center">
  
**Injeção de DLL**: [dll-syringe](https://github.com/OpenByteDev/dll-syringe)  <img src="https://go-skill-icons.vercel.app/api/icons?i=rust&size=32" width="40" />

</h2>

<h2 align="center">
  
**Domain-Driven Design**: [Princípios DDD](https://martinfowler.com/bliki/DomainDrivenDesign.html)  <img src="https://cdn-icons-png.flaticon.com/512/1157/1157109.png" width="40" />

</h2>

<h2 align="center">
  
**Framework egui**: [egui](https://github.com/emilk/egui)  <img src="https://go-skill-icons.vercel.app/api/icons?i=rust&size=32" width="40" />

</h2>
