<div align="center">

<h1 align="center">
  <img src="https://i.imgur.com/Yb7yCeJ.png" width="30" />
  Anti-Interview
</h1>

Controle a visibilidade de janelas durante sessões de compartilhamento de tela com integração em nível de sistema, com foco em privacidade e proteção de dados sensíveis (LGPD).

</div>

---

<h1 align="center">
  <img src="https://i.imgur.com/dwyUWDH.gif" width="30"/> Funcionalidades
</h1>

* **Ocultação de Janelas**: Controle a visibilidade de janelas durante sessões de compartilhamento de tela com integração em nível de sistema, com foco em privacidade e proteção de dados sensíveis (LGPD).
* **Manipulação de Processos (Experimental)**: Técnicas avançadas de interação com processos do sistema para fins de pesquisa e estudo de comportamento do Windows
* **Bandeja do Sistema**: Minimize para a bandeja com ícone nativo
* **Visualização ao Vivo**: Preview em tempo real com captura de tela
* **Menu Animado**: Menu lateral expansível com animação suave
* **Temas**: Alterne entre tema claro e escuro
* **Atalhos**: Teclas de atalho personalizáveis
* **Multi-Monitor**: Suporte completo para múltiplos monitores
* **Técnicas Avançadas**: 12 funções de manipulação de janelas
* **Hooks Especializados**: 6 DLLs de hook para processos específicos
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
* Integração de baixo nível com processos Windows
* Microsoft Detours (API Hooking)
* Framework egui
* Integração com Bandeja do Sistema
* CMake + vcpkg

---

<h1 align="center">
  <img src="https://i.imgur.com/dwyUWDH.gif" width="50" />
  Demonstração:
</h1>

Durante gravações ou testes:

<p align="center">
  <img src="https://i.imgur.com/rMrv8Ae.png" width="600" alt="O que você vê" />
  <img src="https://i.imgur.com/TaXoxvb.png" width="600" alt="O que eles veem" />
</p>

**Esquerda**: Sua visão local | **Direita**: O que outros veem durante o compartilhamento

Janelas selecionadas são ocultadas das APIs de captura de tela, tornando-as invisíveis para os participantes da reunião enquanto permanecem totalmente funcionais.

**Compatível com**: Qualquer aplicação que use APIs de captura de tela do Windows.

---

## Visão Geral

Durante sessões de compartilhamento de tela (reuniões, demonstrações ou transmissões ao vivo), informações sensíveis ou aplicações privadas podem ser expostas inadvertidamente.

O **Anti-Interview** fornece uma camada de filtragem de janelas em nível de sistema, permitindo que aplicações selecionadas sejam excluídas das APIs de captura de tela, enquanto permanecem totalmente visíveis e utilizáveis localmente.

Isso permite:

- Proteção de informações sensíveis (tokens, credenciais, ferramentas internas)
- Apresentações mais limpas e controladas
- Maior segurança em demonstrações e gravações ao vivo

A aplicação integra-se diretamente com os mecanismos de captura do Windows, garantindo compatibilidade com ferramentas populares como Zoom, Microsoft Teams, Discord e OBS.

---

## Casos de Uso

### Uso legítimo

- Ocultar chaves de API ou credenciais durante sessões de live coding  
- Evitar vazamento acidental de dashboards internos em reuniões  
- Manter aplicações pessoais privadas durante compartilhamento de tela  
- Demonstrar softwares de forma controlada  
- Pesquisar o comportamento e limitações das APIs de captura de tela do Windows  

---

## Aviso Importante

Este projeto é destinado exclusivamente para fins de:

- Privacidade  
- Pesquisa  
- Educação  

**Não deve ser utilizado para:**

- Violar políticas de plataformas  
- Burlar mecanismos de segurança  
- Enganar avaliadores em processos seletivos ou ambientes profissionais  

O uso indevido é de total responsabilidade do usuário.

---
<h1 align="center">
  <img src="https://i.imgur.com/PFZmPWb.gif" width="30" />
  Uso:
</h1>

### Início Rápido | Instalação

1. Execute `anti-interview.exe` como administrador (Baixe os binários primeiro)
2. Navegue pelo menu lateral:
   - **Início**: Visão geral e acesso rápido
   - **Janelas**: Gerenciar janelas ocultas
   - **Configurações**: Personalizar comportamento
   - **Créditos**: Informações sobre o projeto
3. Selecione as janelas para ocultar da lista
4. Inicie sua sessão de compartilhamento de tela
5. Janelas ocultas não aparecerão na tela compartilhada

### Menu Lateral

- Clique no botão **▶/◀** para expandir/recolher o menu
- Menu expandido mostra rótulos de texto
- Menu recolhido mostra apenas ícones com tooltips

### Gerenciar Janelas

1. Vá para a seção **Janelas**
2. Use o filtro para buscar janelas específicas
3. Marque as caixas de seleção para ocultar janelas
4. Janelas ocultas ficam invisíveis para captura de tela
5. Preview mostra como outros verão sua tela

### Configurações

#### Interface
- **Tema Escuro**: Alterna entre tema claro e escuro
- **Ocultar de Alt+Tab**: Remove janelas da lista de tarefas
- **Mostrar Preview**: Exibe preview da área de trabalho

#### Comportamento de Janelas
- **Ignorar Mouse**: Janelas ocultas ficam transparentes ao clique

#### Ocultar Aplicações Comuns
Oculte rapidamente navegadores e IDEs:
- **Navegadores**: Firefox, Microsoft Edge, Google Chrome
- **IDEs**: Visual Studio Code, Visual Studio

#### Interação com Processos do Sistema 
Recurso experimental voltado para estudo de APIs internas do Windows. Não é necessário para o uso principal da aplicação:

- **Notepad.exe**: Oculta Bloco de Notas
- **firefox.exe**: Oculta Firefox e todos os processos filhos
- **msedge.exe**: Oculta Microsoft Edge e todos os processos filhos
- **chrome.exe**: Oculta Google Chrome e todos os processos filhos
- **Code.exe**: Oculta VS Code e todos os processos filhos
- **devenv.exe**: Oculta Visual Studio e todos os processos filhos

**Nota**: Requer privilégios de administrador e Task Manager em execução

#### Testes de Payload
Funções experimentais para testar manipulação de janelas:
- **Minimizar/Maximizar/Restaurar**: Controle de estado da janela
- **Sempre no Topo**: Mantém janela acima de outras
- **Piscar na Barra**: Chama atenção na barra de tarefas
- **Opacidade**: Ajusta transparência (0-255)
- **Ocultar Cursor**: Esconde o cursor do mouse (experimental)

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

<h1 align="center">
  <img src="https://i.imgur.com/O7HwCZt.gif" width="30"/> Roadmap
</h1>

* [x] Ocultação de janelas com injeção de DLL
* [x] Integração com bandeja do sistema
* [x] Preview de captura de tela
* [x] Suporte multi-monitor
* [x] Refatoração com arquitetura DDD
* [x] Gerenciamento de configuração
* [x] Banner animado com GIF
* [x] Menu lateral animado expansível
* [x] Sistema de temas (claro/escuro)
* [x] 12 funções de payload para manipulação de janelas
* [x] Seção de créditos
* [x] Sistema de hooks para Task Manager
* [x] 6 DLLs de hook especializadas (Notepad, Firefox, Edge, Chrome, VS Code, Visual Studio)
* [x] Ocultação rápida de navegadores e IDEs
* [ ] Ocultar Cursor
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
  
**InvisWind - Radiantly**: [Link](https://github.com/radiantly/Invisiwind/tree/main)  <img src="https://go-skill-icons.vercel.app/api/icons?i=github&size=32" width="40" />

</h2>

<h2 align="center">
  
**Integração de baixo nível com processos Windows**: [dll-syringe](https://github.com/OpenByteDev/dll-syringe)  <img src="https://go-skill-icons.vercel.app/api/icons?i=rust&size=32" width="40" />

</h2>

<h2 align="center">
  
**Domain-Driven Design**: [Princípios DDD](https://martinfowler.com/bliki/DomainDrivenDesign.html)  <img src="https://cdn-icons-png.flaticon.com/512/1157/1157109.png" width="40" />

</h2>

<h2 align="center">
  
**Framework egui**: [egui](https://github.com/emilk/egui)  <img src="https://go-skill-icons.vercel.app/api/icons?i=rust&size=32" width="40" />

</h2>

<h2 align="center">
  
**Microsoft Detours**: [Detours](https://github.com/microsoft/Detours)  <img src="https://go-skill-icons.vercel.app/api/icons?i=cpp&size=32" width="40" />

</h2>


<h2 align="center">
  
**Microsoft winternl.h**: [NtQuerySystemInfo](https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntquerysysteminformation?source=post_page-----64043c7c2c4b---------------------------------------)  <img src="https://go-skill-icons.vercel.app/api/icons?i=cpp&size=32" width="40" />

</h2>

<h2 align="center">
  
**Hide Processes in Task Manager (Article) - S12 - 0x12Dark Development**: [Forum](https://medium.com/@s12deff/hide-processes-in-task-manager-64043c7c2c4b)  <img src="https://go-skill-icons.vercel.app/api/icons?i=cpp,c&size=32" width="40" />

</h2>

<h1 align="center">Créditos</h1>

<p align="center">
  <strong>Desenvolvido e Pensado por:</strong><br>
  Matheus & Pyetrah (Designer)<br>
</p>
