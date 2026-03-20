# Anti-Interview v1.0.0 - Notas de Lançamento

**Data de Lançamento:** 20/03/2026

## Visão Geral

Anti-Interview é uma aplicação Windows que oculta janelas de captura de tela e compartilhamento (Zoom, Teams, Discord, OBS) e opcionalmente do Gerenciador de Tarefas.

## Funcionalidades

### Funcionalidades Principais
- **Ocultação de Janelas**: Oculta janelas específicas das APIs de captura de tela
- **Preview de Captura de Tela**: Visualização em tempo real do que outros veem
- **Suporte Multi-Monitor**: Funciona com múltiplos displays
- **Integração com Bandeja do Sistema**: Minimize para a bandeja
- **Configuração Persistente**: Configurações salvas entre sessões

### Funcionalidades Avançadas
- **Hook de API NT**: Oculta processos do Gerenciador de Tarefas (incluindo processos filhos)
- **Ocultação Rápida de Navegadores**: Oculte Firefox, Edge, Chrome com um clique
- **Ocultação Rápida de IDEs**: Oculte VS Code, Visual Studio com um clique
- **Ocultação do Task Manager**: Oculta processos da lista de processos do Gerenciador de Tarefas
- **Ocultação do Alt+Tab**: Remove janelas do alternador de tarefas

### Funcionalidades de Interface
- **Menu Lateral Animado**: Menu de navegação expansível
- **Tema Escuro/Claro**: Alterne entre temas
- **Banner Animado**: Suporte a banner GIF
- **Filtro de Janelas**: Busque e filtre janelas
- **Exibição de Nome de Processo**: Mostra nomes reais dos processos

## Instalação

### Instalador (Recomendado)
1. Baixe `anti-interview-setup-v1.0.0.exe`
2. Execute o instalador como administrador
3. Siga o assistente de instalação
4. Inicie pelo Menu Iniciar ou Área de Trabalho

### Versão Portátil
1. Baixe `anti-interview-v1.0.0-portable.zip`
2. Extraia para qualquer pasta
3. Execute `anti-interview.exe` como administrador (para funcionalidades completas)

## Requisitos do Sistema

- **SO**: Windows 10/11 (64-bit)
- **RAM**: 100 MB mínimo
- **Disco**: 50 MB de espaço livre
- **Privilégios**: Administrador recomendado para funcionalidades completas

## Início Rápido

1. Inicie o Anti-Interview
2. Vá para a seção **Janelas**
3. Selecione as janelas para ocultar
4. Inicie o compartilhamento de tela
5. Janelas ocultas não aparecerão na tela compartilhada

## Configuração Avançada (Hook NT)

Para ocultação completa do Gerenciador de Tarefas:

1. Vá para **Configurações** > **Injetar Hooks no Task Manager**
2. Abra o Gerenciador de Tarefas
3. Marque as caixas de seleção para os processos que deseja ocultar:
   - 📝 Ocultar Notepad.exe
   - 🦊 Ocultar firefox.exe
   - 🌐 Ocultar msedge.exe
   - 🎨 Ocultar chrome.exe
   - 💻 Ocultar Code.exe
   - 🔧 Ocultar devenv.exe
4. Os hooks são injetados no Taskmgr.exe
5. Todos os processos filhos serão ocultados do Gerenciador de Tarefas

**Nota**: Requer privilégios de administrador e Gerenciador de Tarefas em execução.

## Atalhos de Teclado

| Atalho | Ação |
|--------|------|
| `Ctrl+Shift+S` | Capturar tela |
| `Ctrl+Shift+H` | Ocultar janela selecionada |
| `Ctrl+Shift+I` | Mostrar/Ocultar GUI |

## Limitações Conhecidas

- A própria janela do Gerenciador de Tarefas requer privilégios de administrador para ocultar
- Alguns processos protegidos do sistema não podem ser ocultados
- Hooks NT requerem que os arquivos `hook_*.dll` estejam presentes
- Desmarcar checkbox não remove o hook (requer reiniciar o Gerenciador de Tarefas)

## Compatibilidade

**Testado com:**
- Zoom
- Microsoft Teams
- Discord
- OBS Studio
- Google Meet
- Skype

**Navegadores:**
- Google Chrome
- Microsoft Edge
- Mozilla Firefox

**IDEs:**
- Visual Studio Code
- Visual Studio 2019/2022

## Detalhes Técnicos

- **Linguagem**: Rust
- **Framework de UI**: egui
- **Arquitetura**: Domain-Driven Design (DDD)
- **Captura de Tela**: Windows API (SetWindowDisplayAffinity)
- **Ocultação de Processos**: Hooking de API NT (NtQuerySystemInformation)
- **Hooking**: Microsoft Detours

## Arquivos Incluídos

```
anti-interview.exe           - Aplicação principal
utils.dll                    - DLL de payload para manipulação de janelas
hook_notepad.dll             - Hook para ocultar Notepad.exe
hook_firefox.dll             - Hook para ocultar firefox.exe
hook_edge.dll                - Hook para ocultar msedge.exe
hook_chrome.dll              - Hook para ocultar chrome.exe
hook_vscode.dll              - Hook para ocultar Code.exe
hook_visualstudio.dll        - Hook para ocultar devenv.exe
README.md                    - Documentação
LICENSE                      - Informações de licença
RELEASE_NOTES.md             - Notas de lançamento (inglês)
USER_GUIDE.md                - Guia do usuário
HOOK_DLL_SYSTEM.md           - Documentação técnica do sistema de hooks
assets/                      - Ícones, fontes, banner
```

## Configuração

Configuração armazenada em:
```
C:\Users\{username}\AppData\Roaming\AntiInterview\config.toml
```

## Solução de Problemas

### Erros "Acesso Negado"
- Execute como administrador
- Alguns processos do sistema são protegidos

### Hook NT não funciona
- Certifique-se de que os arquivos `hook_*.dll` estão no mesmo diretório do executável
- Marque as caixas de seleção em Configurações > Injetar Hooks no Task Manager
- Certifique-se de que o Gerenciador de Tarefas está em execução

### Janelas não estão ocultando
- Verifique se a janela está na lista
- Atualize a lista de janelas
- Reinicie a aplicação

## Suporte

Para problemas, perguntas ou contribuições:
- GitHub: [URL do Repositório]
- Email: [Email de Contato]

## Créditos

**Desenvolvido por:** Matheus & Pyetrah

**Tecnologias:**
- Linguagem de Programação Rust
- Windows API
- Framework egui
- dll-syringe
- Microsoft Detours

**Referências:**
- [InvisWind por Radiantly](https://github.com/radiantly/Invisiwind)
- [Windows SetWindowDisplayAffinity](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowdisplayaffinity)
- [Microsoft Detours](https://github.com/microsoft/Detours)

## Licença

Veja o arquivo LICENSE para detalhes.

## Changelog

### v1.0.0 (20/03/2026)
- Lançamento inicial
- Ocultação de janelas da captura de tela
- Hook de API NT para ocultação do Gerenciador de Tarefas
- 6 DLLs de hook especializadas para processos específicos
- Ocultação rápida de navegador/IDE
- Suporte multi-monitor
- Integração com bandeja do sistema
- Tema escuro/claro
- UI animada
- Configuração persistente
- 12 funções de payload para manipulação de janelas
- Arquitetura DDD limpa e manutenível
