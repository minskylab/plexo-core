<p align="center">
  <img alt="Plexo | Open-Source and AI-Powered Project Management System for modern innovators" src="/public/plexo_gh_banner.png" style="width:auto; height:320px">
</p>

<p align="center">
 ➡️ <a href="https://demo.plexo.app/">Live Demo</a> ⬅️
</p>

<p align="center">
  <img alt="GitHub Workflow Status" src="https://img.shields.io/github/actions/workflow/status/minskylab/plexo-core/registry-docker.yml">
  <img alt="Docker Image Size (latest by date)" src="https://img.shields.io/docker/image-size/minskylab/plexo">
  <img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/minskylab/plexo-core">
  <img alt="GitHub issues" src="https://img.shields.io/github/issues/minskylab/plexo-core">
  <img alt="GitHub" src="https://img.shields.io/github/license/minskylab/plexo-core">
  <img alt="Twitter Follow" src="https://img.shields.io/twitter/follow/plexoapp?style=social">
  <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/minskylab/plexo-core?style=social"> 
</p>

# Plexo

Plexo is an innovative, open-source project management platform that harnesses the power of AI to streamline the way you work. Designed to simplify task tracking within projects and teams, Plexo cuts through the complexities of traditional project management, replacing them with intuitive and efficient solutions.

Plexo's advanced AI functionalities are the heart of the platform. The AI autonomously generates tasks necessary for project completion, taking into account the project's requirements, deadlines, and the team's capabilities. This intelligent task creation optimizes the planning process, freeing up your team to focus on the core work at hand.

<p align="center">
  <img alt="Plexo Platform Screenshot" src="/public/plexo_platform_demo_2.png" height="512" width="auto" >
</p>

Plexo is designed to serve as a benchmark for project execution and description, promoting seamless interoperability among diverse teams and organizations. This is achieved by adhering to the principle that system designs reflect their organization's communication structure. This principle, known as Conway's Law, is deeply ingrained in Plexo, making it a highly effective tool for mirroring and enhancing team communication.

Adopt Plexo to enhance your software project planning and elevate team synergy.

## Features

- 🧠 **AI-Powered Suggestions**: Plexo provides intelligent suggestions to aid in project planning and task management.

- 📈 **Active Task Tracking**: Follow the progress of tasks/issues in real-time within a project, team, or individual context.

- 🤖 **Autonomous Task Creation**: Plexo can autonomously generate tasks necessary for project completion, optimizing the planning process.

- 🤝 **Seamless Collaboration**: Plexo facilitates collaboration between team members, streamlining communication and increasing efficiency.

- 🔀 **Interoperability**: Designed to become a standard in project description and execution, Plexo aims to enhance interoperability between different organizations and teams.

- 🔓 **Open-Source and Free Forever**: Plexo is committed to remaining an open-source project, fostering a community of contributors and users.

- 🍃 **Lightweight and Self-Hosted**: Plexo is designed to be lightweight and self-hostable, reducing dependencies and providing flexibility.

- 🔄 **Conway's Law Inspired**: Plexo is modeled on the principle that organizations design systems are analogous to their communication structure, thus mirroring team communication in its project management system.

## Quick Start

You can try our demo [here](https://demo.plexo.app/). And if you want to deploy your own instance of Plexo-core, actually you need a Postgresql database, a OpenAI API Key and a Github OAuth app. Then you can run the following command:

```bash
docker run \
    -p 8080:8080 \
    -e DATABASE_URL="postgres://postgres:postgres@localhost:5432/plexo" \
    -e OPENAI_API_KEY="<your-openai-api-key>" \
    -e GITHUB_CLIENT_ID="<your-github-client-id>" \
    -e GITHUB_CLIENT_SECRET="<your-github-client-secret>" \
    -e JWT_ACCESS_TOKEN_SECRET="<your-github-client-secret>" \
    -e JWT_REFRESH_TOKEN_SECRET="<your-github-client-secret>" \
    minskylab/plexo
```

⚠️ We're working on a way to deploy Plexo-core without the need of a Github OAuth app. If you want to contribute, please check [this issue](https://github.com/minskylab/plexo-core/issues/9).

<!-- ## Technologies and Programming Languages

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

Aditionally the loaders folder includes an implementation of a data loader to soften the amount of requests made to the database. -->

## Contribution

We welcome all contributions to the Plexo project! Whether you're interested in fixing bugs, adding new features, or improving documentation, your input is greatly valued.

## License

Plexo-core is released under both the MIT and Apache 2.0 licenses. Users are free to use, modify, and distribute the software. Comments and feedback are greatly appreciated.
