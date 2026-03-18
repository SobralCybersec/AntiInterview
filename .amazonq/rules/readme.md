# Rules

- The Readme MUST Follow this documentation:

```md
<div align="center">

<h1 align="center">
  <img src="https://i.imgur.com/dGls2xX.png" width="50" />
  The Apothecary Diaries - Farmácia Kusuriya
</h1>

* Sistema de entrega farmacêutica com validação de receita
* Backend em Java utilizando arquitetura DDD
* Integração com farmácias e sistema de pedidos
* Upload e validação de receitas médicas

---

<h1 align="center">
  <img src="https://i.imgur.com/wetM5VP.png" width="50"/>
  Documentação (Notion)
</h1>
 <img src="https://i.imgur.com/SNBDaws.png" />
 
 ---
 
 <img src="https://i.imgur.com/8wFewjZ.png" />
</div>

---

<h1 align="center">
  <img src="https://i.imgur.com/dwyUWDH.gif" width="30"/> Features
</h1>

* **Sistema de Cadastro**: registro de pacientes com dados básicos
* **Login Simples**: autenticação de usuários
* **Envio de Receita**: upload de receita médica (PDF ou imagem)
* **Armazenamento Seguro**: backup das receitas
* **Validação de Receita**: verificação básica de prescrições médicas
* **Solicitação de Medicamentos**: pacientes podem solicitar medicamentos
* **Sistema de Pedidos**: criação e gerenciamento de pedidos
* **Gestão de Entregas**: controle de status da entrega
* **Integração com Farmácias**: consulta de estoque e disponibilidade
* **API de Entregas**: suporte para motoboy ou drones (opcional)

---

<h1 align="center">
  <img src="https://i.imgur.com/eu3StDB.gif" width="30"/> Tech Stack / Tecnologias
</h1>

<p align="center">
  <img src="https://go-skill-icons.vercel.app/api/icons?i=java,spring,swagger,maven,postgres,docker,aws&size=64" />
</p>

---

* Java 21
* Spring Boot
* Maven
* Spring Data JPA
* PostgreSQL
* Swagger / OpenAPI
* AWS S3 (armazenamento de receitas)
* Docker (opcional)

---

<h1 align="center">
  <img src="https://cdn-icons-png.flaticon.com/512/1157/1157109.png" width="30"/> Architecture / Arquitetura
</h1>

<div align="center">
  
• O projeto segue princípios de **DDD (Domain Driven Design)** e **Clean Architecture**.
• Fundamentos com SOLID.
• Documentação completa estará disponível conforme o andar do projeto.
</div>

---

<h1 align="center">
  <img src="https://cdn-icons-png.flaticon.com/512/709/709496.png" width="30"/> System Flow
</h1>

<div align="center">
  
```
Paciente cria conta
        ↓
Paciente envia receita médica
        ↓
Sistema valida receita
        ↓
Paciente solicita medicamento
        ↓
Sistema cria pedido
        ↓
Farmácia confirma disponibilidade
        ↓
Entrega é criada
        ↓
Paciente acompanha status da entrega
```
</div>

---

<h1 align="center">
  <img src="https://i.imgur.com/O7HwCZt.gif" width="30"/> Roadmap
</h1>

* [ ] Cadastro de pacientes
* [ ] Upload de receita
* [ ] Validação de receita
* [ ] Sistema de pedidos
* [ ] Integração com farmácias
* [ ] Sistema de entregas
* [ ] Integração com drones (opcional)
* [ ] Dashboard administrativo

---


<h1 align="center"><img src="https://i.imgur.com/6nSJzZ2.gif" width="35"/> Referências e Documentações utilizadas</h1>

<h2 align="center">
  
**Spring Boot Docs**: [Link](https://docs.spring.io/spring-boot/index.html)  <img src="https://go-skill-icons.vercel.app/api/icons?i=spring&size=64" width="40" />

</h2>

<h2 align="center">
  
**AWS S3:** [Link](https://docs.spring.io/spring-cloud-config/reference/server/environment-repository/aws-s3-backend.html)  <img src="https://go-skill-icons.vercel.app/api/icons?i=aws&size=64" width="40" />

</h2>

<h2 align="center">
  
**Drone API(Mavlink):** [Link](https://github.com/mavlink/MAVSDK-Java)  <img src="https://go-skill-icons.vercel.app/api/icons?i=maven&size=32" width="40" />

</h2>

<h2 align="center">
  
**Swagger API:** [Link](https://swagger.io/docs/)  <img src="https://go-skill-icons.vercel.app/api/icons?i=swagger&size=35" witdh="40" /> 

</h2>

<h2 align="center">
  
**Database / Banco de Dados PostgreSql:**: [Link](https://willian-kaminski.medium.com/desenvolvendo-uma-rest-api-com-spring-boot-postgresql-parte-1-61bf0d864965)  <img src="https://go-skill-icons.vercel.app/api/icons?i=postgres&size=32" width="40" /> 

</h2>

<h2 align="center">
  
**Segurança:** [Bycrpt](https://docs.spring.io/spring-security/reference/api/java/org/springframework/security/crypto/bcrypt/BCrypt.html) <img src="https://go-skill-icons.vercel.app/api/icons?i=springsecurity&size=32" width="40" /> 

</h2>

<h2 align="center">
  
**Docker:** [Link](https://docs.docker.com/)  <img src="https://go-skill-icons.vercel.app/api/icons?i=docker&size=32" width="40" /> 

</h2>
```