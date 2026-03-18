# Clean Code & High-Performance Guidelines for Rust (AI Coding Rule)

Version: 1.0  
Date: 2026-03-18  
Author: Matheus

---

You are generating Rust code for a production environment focused on system-level programming and high-performance automation.

The following rules are mandatory:

- You Must Follow reference projects: https://github.com/aamitn/winhider, https://github.com/radiantly/Invisiwind, https://github.com/Mx0M/speech-to-text-rust
- You Must Follow the documentation style defined here: https://raw.githubusercontent.com/SobralCybersec/AWS-QrCodeGenerator/refs/heads/master/README.md
- readable
- maintainable
- modular
- high-performance
- memory-safe
- aligned with Rust best practices
- Do not describe obvious code behavior in comments.
- Comments must only explain non-obvious decisions.

# 1. Purpose

This rule defines coding standards to ensure that generated Rust code is:

- high-performance
- memory-efficient
- modular
- scalable
- secure
- real-time capable
- aligned with Rust idioms
- Do not describe obvious code behavior in comments.
- Comments must only explain non-obvious decisions.
- You Must Follow the documentation style defined here: https://raw.githubusercontent.com/SobralCybersec/AWS-QrCodeGenerator/refs/heads/master/README.md

The goal is to produce system-level code optimized for automation, low latency, and direct OS integration.

---

# 2. General Guidelines

The AI MUST follow these principles when generating Rust code.

| # | Rule | Reason |
|---|------|-------|
| 1 | Never use emojis in comments, logs, or documentation | Keeps code professional |
| 2 | Avoid redundant comments | Only explain complex logic |
| 3 | Avoid unnecessary println! in production | Use proper logging (tracing) |
| 4 | Use clear and meaningful variable names | Improves readability |
| 5 | Follow Rust idioms strictly | Improves maintainability |
| 6 | Each function must have a single responsibility | Reduces complexity |
| 7 | Keep code concise and readable | Avoid verbose patterns |
| 8 | Avoid unnecessary allocations | Improves performance |
| 9 | Prefer standard library when possible | Avoid unnecessary dependencies |
| 10 | Optimize for zero-copy operations | Minimize memory overhead |
| 11 | Never use unwrap() or expect() in production | Use proper error handling |
| 12 | Avoid typical AI documentation style | Follow the reference documentation style |

---

# 3. Control Flow Rules (Rust)

The AI must follow these control flow practices.

## Prefer early returns

Use early returns to reduce nesting.

Bad example:

```rust
fn processar(valor: Option<i32>) -> Result<i32, Error> {
    if let Some(v) = valor {
        if v > 0 {
            Ok(v * 2)
        } else {
            Err(Error::InvalidValue)
        }
    } else {
        Err(Error::NoneValue)
    }
}
```

Preferred:

```rust
fn processar(valor: Option<i32>) -> Result<i32, Error> {
    let v = valor.ok_or(Error::NoneValue)?;
    
    if v <= 0 {
        return Err(Error::InvalidValue);
    }
    
    Ok(v * 2)
}
```

## Prefer match over if-else chains

Bad example:

```rust
if status == "A" {
    processar_a();
} else if status == "B" {
    processar_b();
} else if status == "C" {
    processar_c();
}
```

Preferred:

```rust
match status {
    "A" => processar_a(),
    "B" => processar_b(),
    "C" => processar_c(),
    _ => {}
}
```

## Use pattern matching effectively

Preferred pattern:

```rust
match resultado {
    Ok(valor) => processar(valor),
    Err(e) => {
        tracing::error!("Erro: {:?}", e);
        return;
    }
}
```

---

# 4. Error Handling Rules (Rust)

Use proper error handling with Result and custom error types.

Guidelines:

1. Never use unwrap() or expect() in production code.
2. Use ? operator for error propagation.
3. Define custom error types with thiserror.
4. Use anyhow for application-level errors.

Bad example:

```rust
let arquivo = File::open("config.toml").unwrap();
```

Preferred:

```rust
use anyhow::{Context, Result};

fn carregar_config() -> Result<Config> {
    let conteudo = std::fs::read_to_string("config.toml")
        .context("Falha ao ler arquivo de configuração")?;
    
    toml::from_str(&conteudo)
        .context("Falha ao parsear configuração")
}
```

Custom error types:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AutomationError {
    #[error("Janela não encontrada: {0}")]
    WindowNotFound(String),
    
    #[error("Falha na captura de áudio")]
    AudioCaptureError,
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

---

# 5. Rust Principles (Mandatory)

## Ownership and Borrowing

Prefer borrowing over cloning.

Bad:

```rust
fn processar(dados: Vec<u8>) -> Vec<u8> {
    dados.clone()
}
```

Good:

```rust
fn processar(dados: &[u8]) -> Vec<u8> {
    dados.to_vec()
}
```

## Lifetimes

Use explicit lifetimes only when necessary.

```rust
struct WindowHandler<'a> {
    titulo: &'a str,
}
```

## Zero-Cost Abstractions

Prefer iterators over manual loops.

Bad:

```rust
let mut resultado = Vec::new();
for i in 0..dados.len() {
    if dados[i] > 10 {
        resultado.push(dados[i] * 2);
    }
}
```

Good:

```rust
let resultado: Vec<_> = dados.iter()
    .filter(|&&x| x > 10)
    .map(|&x| x * 2)
    .collect();
```

---

# 6. Performance Rules

## Memory Allocation

1. Pre-allocate vectors when size is known.
2. Use Vec::with_capacity() instead of Vec::new().
3. Reuse buffers when possible.

Example:

```rust
let mut buffer = Vec::with_capacity(1024);
```

## Avoid Unnecessary Cloning

Bad:

```rust
fn processar(texto: String) -> String {
    texto.clone()
}
```

Good:

```rust
fn processar(texto: &str) -> String {
    texto.to_string()
}
```

## Use References

Prefer &str over String, &[T] over Vec<T> in function parameters.

```rust
fn analisar(texto: &str) -> usize {
    texto.len()
}
```

---

# 7. Concurrency Rules

## Use Tokio for Async

All async code must use tokio runtime.

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let resultado = processar_async().await?;
    Ok(())
}
```

## Prefer Channels over Mutex

Bad:

```rust
let dados = Arc::new(Mutex::new(Vec::new()));
```

Good:

```rust
let (tx, rx) = tokio::sync::mpsc::channel(100);
```

## Use Arc for Shared State

```rust
use std::sync::Arc;

let estado = Arc::new(Estado::new());
let estado_clone = Arc::clone(&estado);
```

---

# 8. Project Structure (Mandatory)

```
src/
├── main.rs
├── app/
│   ├── mod.rs
│   ├── state.rs
│   └── config.rs
├── ui/
│   ├── mod.rs
│   ├── theme.rs
│   └── components.rs
├── system/
│   ├── mod.rs
│   ├── window.rs
│   ├── hotkeys.rs
│   └── screenshot.rs
├── audio/
│   ├── mod.rs
│   ├── recorder.rs
│   └── stt.rs
├── automation/
│   ├── mod.rs
│   └── pipeline.rs
└── utils/
    ├── mod.rs
    ├── logger.rs
    └── errors.rs
```

---

# 9. Naming Conventions

Use Rust naming conventions strictly.

Good examples:

```rust
let numero_pares: Vec<i32>;
let window_handler: WindowHandler;
fn obter_janelas_ativas() -> Vec<Window>;
struct AudioRecorder;
enum TipoAcao;
const MAX_BUFFER_SIZE: usize = 1024;
```

Bad examples:

```rust
let tmp;
let data1;
let x;
fn func();
```

---

# 10. Logging Rules

Use tracing for structured logging.

```rust
use tracing::{info, warn, error, debug};

#[tracing::instrument]
fn processar_janela(hwnd: u32) -> Result<()> {
    debug!("Processando janela: {}", hwnd);
    
    match ocultar_janela(hwnd) {
        Ok(_) => info!("Janela ocultada com sucesso"),
        Err(e) => error!("Falha ao ocultar janela: {:?}", e),
    }
    
    Ok(())
}
```

Never use println! in production:

Bad:

```rust
println!("Processando...");
```

Good:

```rust
tracing::info!("Processando...");
```

---

# 11. Testing Strategy (Mandatory)

## Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocultar_janela() {
        let resultado = validar_hwnd(12345);
        assert!(resultado.is_ok());
    }
}
```

## Integration Tests

Create tests/ directory:

```rust
#[tokio::test]
async fn test_pipeline_completo() {
    let pipeline = Pipeline::new();
    let resultado = pipeline.executar().await;
    assert!(resultado.is_ok());
}
```

## Mocking

Use mockall for mocking:

```rust
use mockall::automock;

#[automock]
trait WindowService {
    fn ocultar(&self, hwnd: u32) -> Result<()>;
}
```

---

# 12. Dependencies Management

Prefer minimal dependencies.

Essential crates:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
tracing = "0.1"
tracing-subscriber = "0.3"
anyhow = "1"
thiserror = "1"
```

---

# 13. Unsafe Code Rules

Use unsafe only when absolutely necessary.

Guidelines:

1. Document why unsafe is needed.
2. Minimize unsafe blocks.
3. Encapsulate unsafe in safe abstractions.

Example:

```rust
pub fn set_window_visibility(hwnd: HWND, hide: bool) -> bool {
    let affinity = if hide {
        WDA_EXCLUDEFROMCAPTURE
    } else {
        WDA_NONE
    };
    
    // SAFETY: hwnd is validated before this call
    let result = unsafe { SetWindowDisplayAffinity(hwnd, affinity) };
    
    !result.is_err()
}
```

---

# 14. Documentation Rules

Use rustdoc comments:

```rust
/// Oculta uma janela do sistema.
///
/// # Arguments
///
/// * `hwnd` - Handle da janela
/// * `hide` - true para ocultar, false para mostrar
///
/// # Returns
///
/// Result indicando sucesso ou falha
///
/// # Example
///
/// ```
/// let resultado = ocultar_janela(12345, true);
/// ```
pub fn ocultar_janela(hwnd: u32, hide: bool) -> Result<()> {
    // implementação
}
```

---

# 15. Configuration Management

Use config files:

```toml
# config.toml
[hotkeys]
screenshot = "Ctrl+Shift+S"
speech_to_text = "Ctrl+Shift+V"
hide_window = "Ctrl+Shift+H"

[stt]
model = "base"
language = "pt"

[ui]
theme = "dark"
```

Load with serde:

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    hotkeys: Hotkeys,
    stt: SttConfig,
    ui: UiConfig,
}

fn carregar_config() -> Result<Config> {
    let conteudo = std::fs::read_to_string("config.toml")?;
    Ok(toml::from_str(&conteudo)?)
}
```

---

# 16. System Integration Rules

## Windows API

Use windows crate properly:

```rust
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{
        SetWindowDisplayAffinity,
        WDA_EXCLUDEFROMCAPTURE,
    },
};

pub fn hide_from_capture(hwnd: HWND) -> Result<()> {
    unsafe {
        SetWindowDisplayAffinity(hwnd, WDA_EXCLUDEFROMCAPTURE)
            .map_err(|e| anyhow!("Falha ao ocultar janela: {:?}", e))?;
    }
    Ok(())
}
```

---

# 17. Async Patterns

## Spawning Tasks

```rust
tokio::spawn(async move {
    processar_audio().await
});
```

## Timeouts

```rust
use tokio::time::{timeout, Duration};

let resultado = timeout(
    Duration::from_secs(5),
    operacao_longa()
).await?;
```

## Select

```rust
tokio::select! {
    resultado = operacao_a() => {
        processar_a(resultado);
    }
    resultado = operacao_b() => {
        processar_b(resultado);
    }
}
```

---

# 18. Code Style Rules

The AI must produce code that is:

- minimal
- readable
- maintainable
- idiomatic Rust

Avoid:

- unnecessary comments
- redundant cloning
- nested if statements
- unnecessary complexity
- unwrap() and expect()

---

# 19. Example

Bad example (verbose, unsafe):

```rust
fn processar_numeros(numeros: Vec<i32>) -> Vec<i32> {
    let mut resultado = Vec::new();
    for i in 0..numeros.len() {
        let numero = numeros.get(i).unwrap();
        if numero % 2 == 0 {
            resultado.push(numero * 2);
        }
    }
    resultado
}
```

Preferred:

```rust
fn processar_numeros(numeros: &[i32]) -> Vec<i32> {
    numeros.iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * 2)
        .collect()
}
```

---

# 20. Final Instruction for AI

When generating Rust code:

1. Follow Rust idioms strictly.
2. Never use unwrap() or expect() in production.
3. Use proper error handling with Result and ?.
4. Prefer borrowing over cloning.
5. Use async/await with tokio.
6. Write code like an experienced Rust systems programmer.
7. Optimize for performance and memory efficiency.
8. Do not generate verbose AI-style code.
9. Always consider zero-cost abstractions.
10. Use tracing for logging, never println!.
