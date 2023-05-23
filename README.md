<!-- <picture>
  <source media="(prefers-color-scheme: dark)" srcset="/public/plexo_logo_white_text.svg">
  <source media="(prefers-color-scheme: light)" srcset="/public/plexo_logo_black_text.svg">
  <img alt="Plexo logoype" src="/public/plexo_logo_black_text.svg">
</picture> -->

<img alt="Plexo | Open-Source and AI-Powered Project Management System for modern innovators" src="/public/plexo_gh_banner.png" height="420" align="center">

# Plexo

Plexo is an open-source solution reshaping project management with AI. It simplifies task tracking within projects and teams, replacing traditional complexities. Its AI functionalities autonomously create project tasks and provide valuable suggestions, helping teams to prioritize their core work.

More than a tool, Plexo is a benchmark for project execution and description, fostering interoperability amongst diverse teams and organizations. Drawing on the principle that system designs reflect their organization's communication structure, Plexo stands as evidence of this theory, exemplifying organizational efficiency.

Adopt Plexo to enhance your software project planning and elevate team synergy.

## Main Features and Functionalities of Plexo-Core

Plexo-Core specifically is the backend part of the Plexo-platform project. Its purpose is to serve as a connection between the frontend and the Hashura database. Plexo-core has built 7 core "objects" around which the platform works. They are "tasks", "members", "teams", "projects", "labels", and "organizations". Around all of them, the features are distributed as an interconnection between the basic conceptual relationships every core object has. For example, tasks have a due date, a leader, members as assignees, labels, etc. Projects have also due date, teams associated, an owner, a leader, etc.

## Live Demo

<img align="right" height="255" src="/public/plexo-live-1.svg" alt="Image of tasks on board view" title="Board view Plexo">

<img align="right" height="255" src="/public/plexo-live-2.svg" alt="Image of task creation with tasks list view on the back" title="Task creation Plexo">

<img align="right" height="255" src="/public/plexo-live-3.svg" alt="View of one of the projects" title="In project view Plexo">

**Try our live demo!** [Live demo](https://demo.plexo.app/)

If you find any bug or are eager to ask for a feature, create a github issue [here] (https://github.com/minskylab/plexo-core/issues)

## Technologies and Programming Languages

The project uses Rust as its language and the other main functional technologies are async GraphQL and Hashura with a Postgresql database. On the other hand, Docker is used to deploy other instances.

## System Requirements

Plexo-core is a lightweight program thanks to how rust works, and does not require almost any system resources.

## Dependencies and Prerequisites

Before using Plexo-core, users need to install Docker.

## Installation and Usage

To install and run Plexo-core on their machines, users can follow these steps:

1. Install Docker on your machine if it's not already installed.
2. Pull the Plexo-core Docker image from the repository.
3. Run the Docker image.

## Usage Instructions and Examples

To use Plexo-core, users can run a GraphQL playground and test the queries, mutations, and subscriptions.
If you are using a local deployment of the project go to [0.0.0.0:8080/playground](http://0.0.0.0:8080/playground) or [localhost:8080/playground](http://localhost:8080/playground).

# Development Progress and Roadmap

- [x] User Creation
- [x] Creation, update and deletion of basic objects
- [x] Sub-queries for each object
- [x] Async GraphQL dataloader
- [ ] Real-time Subscriptions
- [ ] AI Suggestions
- [ ] Automatic task creation

## How it is designed, for devs

Plexo-core as a whole runs around certain objects. This ibjects have queries, sub-queries, mutations and subscriptions set-up around them as possible interactions.

The objects are:

- Labels
- Members
- Projects
- Tasks
- Teams

General queries, mutations and subscriptions can be found on those files. On the other hand specific, sub-quieries for each object can be found inside each respective object file.

Aditionally the loaders folder includes an implementation of a data loader to soften the amount of requests made to the database.

## License

Plexo-core is released under both the MIT and Apache 2.0 licenses. Users are free to use, modify, and distribute the software. Comments and feedback are greatly appreciated.
