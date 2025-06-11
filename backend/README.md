# Backend

Este diretório contém o código do servidor e a lógica de backend do sistema Vektra-3D.

## Estrutura

- `src/`: Código fonte do backend
  - `controllers/`: Controladores da API
  - `models/`: Modelos de dados
  - `routes/`: Definição de rotas
  - `services/`: Serviços e lógica de negócio
  - `middlewares/`: Middlewares da aplicação
  - `config/`: Configurações do servidor
  - `utils/`: Utilitários específicos do backend

## Tecnologias

- Node.js
- Express.js
- TypeScript
- Prisma (ORM)
- PostgreSQL

## Configuração

1. Instale as dependências:
```bash
npm install
```

2. Configure as variáveis de ambiente:
```bash
cp .env.example .env
```

3. Execute as migrações do banco de dados:
```bash
npx prisma migrate dev
```

4. Inicie o servidor em modo desenvolvimento:
```bash
npm run dev
```

## API

A documentação completa da API está disponível em `/api-docs` quando o servidor estiver em execução. 