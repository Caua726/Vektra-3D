# Frontend

Este diretório contém a interface do usuário e a lógica do cliente do sistema Vektra-3D.

## Estrutura

- `src/`: Código fonte do frontend
  - `components/`: Componentes React
  - `pages/`: Páginas da aplicação
  - `hooks/`: Custom hooks React
  - `contexts/`: Contextos React
  - `services/`: Serviços de API
  - `styles/`: Estilos e temas
  - `utils/`: Funções utilitárias
  - `assets/`: Recursos estáticos

## Tecnologias

- React
- TypeScript
- Next.js
- Tailwind CSS
- Three.js (para renderização 3D)

## Configuração

1. Instale as dependências:
```bash
npm install
```

2. Configure as variáveis de ambiente:
```bash
cp .env.example .env
```

3. Inicie o servidor de desenvolvimento:
```bash
npm run dev
```

## Desenvolvimento

- O servidor de desenvolvimento roda em `http://localhost:3000`
- Hot-reload está habilitado por padrão
- Linting e formatação automática configurados com ESLint e Prettier

## Build

Para criar uma build de produção:

```bash
npm run build
```

Para iniciar a versão de produção:

```bash
npm start
``` 