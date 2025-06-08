# Overview

## 1. Introduction and Goals

Mosquittauri is an Application designed to be a pretty Interface for MQTT Communication.

## 1.1 Quality Goals

- The Application should start quickly.
- Settings should be persisted between executions.
- The Software should be pleasing to look at.
- Publishing and Sending should be done independantly.
- Multiple Topics can be subscribed at once.

## 2. Constraints

- The Application should run under Windows.
- The Application should contain all necessary dependencies to run.
- The Application should be written in an object-oriented language.

## 3. Context and Scope

### 3.1 Business Context

#### Use Case Diagram

The Diagram below illustrates what the Use Case (or Relationship) between Mosquittauri, it's User, and the MQTT Broker are, including the relationship between Components in MSQT.

#### TODO: USE CASE DIAGRAM

## 4. Solution Strategy

### 4.1 React Frontend

The React frontend provides a dynamic, component-based user interface for the application. React allows for efficient rendering and state management, making it ideal for building modern, interactive user interfaces. In this project, the React frontend interacts with the Rust backend through Tauri commands, enabling native functionality.

For more information about React, visit the [official React website](https://reactjs.org/).

### 4.2 Tauri/Rust Backend

The **Rust** backend, which **Tauri** provides, powers the native functionality of the application, allowing seamless communication with the operating system and efficient performance. In this project, Tauri enables the React frontend to interact with Rust through commands, allowing the backend to handle system-level tasks, such as file or registry access, and return results asynchronously.

For more information about Tauri and Rust, visit the [official Tauri website](https://tauri.app/) and the [official Rust website](https://www.rust-lang.org/).

### 4.3 Bundling with Tauri

Tauri is used to bundle the application into a lightweight, cross-platform desktop app. It packages the **React frontend** and **Rust backend** into a single executable with a minimal footprint. Tauri’s bundling process ensures secure, fast, and efficient deployment while keeping system resources usage low.

### 4.4 JSON Storage

Originally developed as a temporary solution until the SQL Integration exists, the JSON Storage provides a simple way to store Settings for the Application. It utilizes multiple JSON Files, that will be generated in the Folder where the MSQT is executed.

### 4.5 SQLite Database

#### TODO: SQL DOKUMENTIEREN

### 5, Building Block View

#### TODO: CLASS DIAGRAM

### 6. Runtime View

#### 6.1 MQTT Communication

The following Diagram shows the Process of how MSQT communicates with the MQTT Broker.

#### TODO: SEQUENCE DIAGRAM

## 7. Deployment View

We use a Github Actions Pipeline to automatically build the Application for multiple Platforms, including Windows, Linux and MacOS. This ensures consistent and reproducible builds, as well as giving us multiple "snapshots" of how the Program behaved throughout versions.

## 8. Concepts

#### TODO: GENUTZTE PATTERNS

## 9. Architectural Decisions

#### TODO: WICHTIGE ENTSCHEIDUNGEN

## 10. Risks and Technical Debts

### 10.1 Risks

| Title                       | Description                                                                                                                                                                                                                                                                                                                                                      |
| --------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| No Developers using Windows | None of the Developers working on Mosquittauri are using Windows, neither for Development, nor much for Private Use. As thus, the Application was not thoroughly tested on Windows until the end. This Risk is mitigated by the fact that the Github Actions Pipeline always builds and tests on Windows as well, but some behavior might be slightly different. |

### 10.2 Technical Debts

| Title                    | Description                                                                                                                                                                                     |
| ------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| No User-Defined ClientID | In Development, we forgot to add a way for the User to set the Client ID that is going to be sent to the Broker (Kind of like a username). We wanted to add this later on, but ran out of time. |

## 11. Glossary

| Abbreviation | Description                                |
| ------------ | ------------------------------------------ |
| MSQT         | The Name of the Application - Mosquittauri |
