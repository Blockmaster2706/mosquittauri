## Introduction

Mosquittauri is a [Next.js](https://nextjs.org/) project bootstrapped with [`create-next-app`](https://github.com/vercel/next.js/tree/canary/packages/create-next-app). It is bundled using the [Tauri](https://tauri.app/) framework, to serve a WebView2 WebApp.

Further Information can be found in the [Service Support Tool Documentation](/docs/Documentation.md) under the docs folder.

## Getting Started

### Installation Process

First of, you need to install [Node.js](https://nodejs.org) Version 22.x in case it's not already installed.

After cloning the Repositories, you should make sure you have all the Dependencies for Tauri installed. These are listed in the [Tauri Documentation](https://tauri.app/v1/guides/getting-started/prerequisites)

Afterwards you need to install yarn if it's not already globally installed:

```bash
npm install yarn
```

Also you need to install the required node packages:

```bash
yarn install
```

Once you have ensured that you ran all those steps, you should be able to start development.

First, run the development server:

```bash
yarn tauri dev
```

Now you can start editing the page by modifying `app/page.tsx`, or the other App Components. The page auto-updates as you edit and save the files.

### Software Dependencies

This project uses [`next/font`](https://nextjs.org/docs/basic-features/font-optimization) to automatically optimize and a Google Font.

The project is built with [Next.js](https://nextjs.org/), a framework that uses [React](https://react.dev/) for creating the user interface and [TypeScript](https://www.typescriptlang.org/) for the server-side code. The application leverages [Tauri](https://tauri.app/) to package its components into a compact, standalone executable. Subsequently, this executable can be compressed into a zip file to allow straightforward sharing and distribution. Tauri works with the [Rust programming language](https://www.rust-lang.org/) to handle its main tasks.

The software uses cargo test for component tests.

## Learn More

To learn more about Next.js, take a look at the following resources:

- [Next.js Documentation](https://nextjs.org/docs) - learn about Next.js features and API.
- [Learn Next.js](https://nextjs.org/learn) - an interactive Next.js tutorial.

You can check out [the Next.js GitHub repository](https://github.com/vercel/next.js/)

The Tauri documentation is available [here](https://tauri.app/)
