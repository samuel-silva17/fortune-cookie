# Fortune Cookie

Este projeto é uma aplicação web simples que exibe uma mensagem de fortuna ao usuário ao clicar em um botão. A aplicação é composta por um **backend em Rust** que acessa a API externa [https://api.adviceslip.com/advice](https://api.adviceslip.com/advice) para obter as mensagens de fortuna, e um **frontend em Nginx** que serve uma página estática. Os contêineres são gerenciados utilizando o Podman.

## Índice

- [Estrutura do Projeto](#estrutura-do-projeto)
- [Pré-requisitos](#pré-requisitos)
- [Instruções de Configuração](#instruções-de-configuração)
  - [Clonando o Repositório](#clonando-o-repositório)
  - [Configurando o Backend](#configurando-o-backend)
  - [Configurando o Frontend](#configurando-o-frontend)
- [Construindo as Imagens Docker](#construindo-as-imagens-docker)
- [Executando a Aplicação](#executando-a-aplicação)
- [Verificando os Logs](#verificando-os-logs)
- [Licença](#licença)

---

## Estrutura do Projeto

```bash
.
├── backend
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── Dockerfile
│   └── src
│       └── main.rs
├── fortune_logs
│   ├── backend
│   │   └── fortune_backend.log
│   └── frontend
│       ├── access.log
│       └── error.log
└── frontend
    ├── Dockerfile
    ├── index.html
    └── nginx.conf
```

- **backend/**: Código-fonte e Dockerfile do backend em Rust.
- **frontend/**: Arquivos estáticos e Dockerfile do frontend em Nginx.
- **fortune_logs/**: Diretório para armazenar os logs do backend e frontend.

---

## Pré-requisitos

Certifique-se de ter as seguintes ferramentas instaladas em seu sistema:

- [Podman](https://podman.io/)
- [Rust](https://www.rust-lang.org/tools/install) (caso queira compilar o backend localmente)
- [Docker](https://www.docker.com/) (opcional, se preferir usar Docker em vez de Podman)

---

## Instruções de Configuração

### Clonando o Repositório

Clone o repositório do projeto para o seu ambiente local:

```bash
git clone https://github.com/seu-usuario/fortune-cookie.git
cd fortune-cookie
```

**Nota:** Substitua `seu-usuario` pelo nome de usuário correto, se aplicável.

### Configurando o Backend

1. Navegue até o diretório do backend:

   ```bash
   cd backend
   ```

2. Revise o arquivo `Cargo.toml` e as dependências do projeto.

3. O arquivo `main.rs` contém o código-fonte do servidor em Rust que fornece as mensagens de fortuna através de uma API REST.

4. **API Externa:** O backend em Rust acessa a API [https://api.adviceslip.com/advice](https://api.adviceslip.com/advice) para obter as mensagens de fortuna dinamicamente.

### Configurando o Frontend

1. Navegue até o diretório do frontend:

   ```bash
   cd ../frontend
   ```

2. O arquivo `index.html` contém a página web que será servida pelo Nginx.

3. O arquivo `nginx.conf` é a configuração personalizada do Nginx para servir o frontend.

---

## Construindo as Imagens Docker

Antes de executar a aplicação, é necessário construir as imagens Docker para o backend e o frontend.

### Construindo a Imagem do Backend

1. Navegue até o diretório do backend:

   ```bash
   cd ../backend
   ```

2. Construa a imagem Docker:

   ```bash
   podman build -t localhost/fortune-backend:latest .
   ```

### Construindo a Imagem do Frontend

1. Navegue até o diretório do frontend:

   ```bash
   cd ../frontend
   ```

2. Construa a imagem Docker:

   ```bash
   podman build -t localhost/fortune-frontend:latest .
   ```

---

## Executando a Aplicação

Utilizaremos o Podman para criar um pod que conterá ambos os contêineres do backend e do frontend.

### Passo 1: Criar os Diretórios de Logs

Crie os diretórios para armazenar os logs do backend e do frontend:

```bash
mkdir -p fortune_logs/backend
mkdir -p fortune_logs/frontend
```

### Passo 2: Criar o Pod

Crie o pod chamado `fortune-pod` mapeando as portas necessárias:

```bash
podman pod create --name fortune-pod -p 8000:80 -p 8080:8080
```

### Passo 3: Executar o Backend

Execute o contêiner do backend com o volume para persistência dos logs:

```bash
podman run -d \
  --name backend \
  --pod fortune-pod \
  -v $(pwd)/fortune_logs/backend:/var/log/fortune_backend:Z \
  -e API_URL=https://api.adviceslip.com/advice \
  localhost/fortune-backend:latest
```

**Explicação:**

- `-v $(pwd)/fortune_logs/backend:/var/log/fortune_backend:Z` monta o diretório de logs do backend do host no contêiner.
- `-e API_URL=https://api.adviceslip.com/advice` define a variável de ambiente `API_URL` com a URL da API externa.

### Passo 4: Executar o Frontend

Execute o contêiner do frontend com o volume para persistência dos logs:

```bash
podman run -d \
  --name frontend \
  --pod fortune-pod \
  -v $(pwd)/fortune_logs/frontend:/var/log/nginx:Z \
  localhost/fortune-frontend:latest
```

**Explicação:**

- `-v $(pwd)/fortune_logs/frontend:/var/log/nginx:Z` monta o diretório de logs do frontend do host no contêiner.

### Passo 5: Testar a Aplicação

Abra o navegador e acesse:

```
http://localhost:8000
```

Clique no botão "Get Your Fortune" para receber uma mensagem de fortuna.

---

## Verificando os Logs

### Logs do Backend

Os logs do backend são armazenados em `fortune_logs/backend`.

Verifique os logs:

```bash
ls fortune_logs/backend
cat fortune_logs/backend/fortune_backend.log
```

### Logs do Frontend

Os logs do frontend são armazenados em `fortune_logs/frontend`.

Verifique os logs:

```bash
ls fortune_logs/frontend
cat fortune_logs/frontend/access.log
cat fortune_logs/frontend/error.log
```

---

## Licença

Este projeto é licenciado sob os termos da licença Apache 2.0. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.

---

**Nota:** Certifique-se de ter o Podman devidamente configurado em seu sistema. Os comandos fornecidos utilizam o Podman, mas podem ser adaptados para o Docker com pequenas modificações.

Se você encontrar algum problema ou tiver dúvidas, sinta-se à vontade para abrir uma issue no repositório do projeto.
