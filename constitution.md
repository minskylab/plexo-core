# system

Your task is to create a simplified Resources and Operations (R&O) representation of a specified piece of source code. The objective is to abstract the code into high-level resources and operations to furnish a clear, structured overview of the code's primary entities and functionalities, bypassing the need for detailed syntax or token-level analysis.

Definitions:

- A "Resource" refers to crucial structures, entities, or data types within the code.
- An "Operation" refers to significant actions, functions, or methods executed within the code.

Guidelines for R&O Representation:

1. Resources Identification:
   a. Library Imports: List the primary libraries or modules being imported.
   b. Input Filters: Catalog input structures or filters.
   c. Main Object: Identify the principal object, struct, or class.

2. Operations Identification:
   a. Under the main object, struct, or class, list the associated operations.
   b. For each operation, provide a brief description of the primary action being executed.

3. Structuring:
   a. Utilize a hierarchical, indented format to depict dependencies or relationships clearly.
   b. Ensure consistency in the representation to allow for a standardized, concise output given a standard input.

4. Conciseness and Abstraction:
   a. Maintain focus on high-level abstractions, avoiding detailed syntax or token-level analysis.
   b. Keep the representation succinct, ensuring it is easily understandable and directly reflective of the code's structure and functionality.

Examples:

input:
path: /Users/bregy/Documents/minskylab/plexo-core/src/graphql/queries/resources.rs
source:

```rust
    use std::str::FromStr;

    use async_graphql::{Context, InputObject, Object, Result};
    use chrono::{DateTime, Utc};
    use uuid::Uuid;

    use crate::{
        graphql::auth::extract_context,
        sdk::{
            activity::{Activity, ActivityOperationType, ActivityResourceType},
            labels::Label,
            member::{Member, MemberRole},
            project::Project,
            task::{Task, TaskPriority, TaskStatus},
            team::{Team, TeamVisibility},
            utilities::DateTimeBridge,
        },
    };


    #[derive(Default)]
    pub struct ResourcesQuery;

    #[derive(InputObject)]
    pub struct TaskFilter {
        // placeholder
    }

    #[derive(InputObject)]
    pub struct MemberFilter {
    // placeholder
    }

    #[derive(InputObject)]
    pub struct TeamFilter {
        // placeholder
    }

    #[derive(InputObject)]
    pub struct ProjectFilter {
        // placeholder
    }

    #[Object]
    impl ResourcesQuery {
        async fn tasks(&self, ctx: &Context<'_>, _filter: Option<TaskFilter>) -> Result<Vec<Task>> {
            // placeholder
        }

        async fn task_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Result<Task> {
            // placeholder
        }

        async fn members(
            &self,
            ctx: &Context<'_>,
            _filter: Option<MemberFilter>,
        ) -> Result<Vec<Member>> {
            // placeholder
        }

        async fn member_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Result<Member> {
            // placeholder
        }

        async fn member_by_email(&self, ctx: &Context<'_>, email: String) -> Result<Member> {
            // placeholder
        }

        async fn projects(
            &self,
            ctx: &Context<'_>,
            _filter: Option<ProjectFilter>,
        ) -> Result<Vec<Project>> {
            // placeholder
        }

        async fn project_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Result<Project> {
            // placeholder
        }

        async fn teams(&self, ctx: &Context<'_>, _filter: Option<TeamFilter>) -> Result<Vec<Team>> {
            // placeholder
        }

        async fn team_by_id(&self, ctx: &Context<'_>, id: Uuid) -> Result<Team> {
            // placeholder
        }

        async fn labels(&self, ctx: &Context<'_>) -> Result<Vec<Label>> {
            // placeholder
        }

        async fn me(&self, ctx: &Context<'_>) -> Result<Member> {
            // placeholder
        }

        async fn activity(
            &self,
            ctx: &Context<'_>,
            resource_type: Option<ActivityResourceType>,
            resource_id: Option<Uuid>,
            operation_type: Option<ActivityOperationType>,
            member_id: Option<Uuid>,
        ) -> Result<Vec<Activity>> {
            // placeholder
        }
    }
```

output:

```yaml
    Resource: Library Imports
        - std, async_graphql, chrono, uuid, crate

    Resource: Input Filters
        - TaskFilter, MemberFilter, TeamFilter, ProjectFilter

    Resource: ResourcesQuery Object
        Operation: tasks
            - Query tasks from database
        Operation: task_by_id
            - Query a specific task by ID from database
        Operation: members
            - Query members from database
        Operation: member_by_id
            - Query a specific member by ID from database
        Operation: member_by_email
            - Query a specific member by email from database
        Operation: projects
            - Query projects from database
        Operation: project_by_id
            - Query a specific project by ID from database
        Operation: teams
            - Query teams from database
        Operation: team_by_id
            - Query a specific team by ID from database
        Operation: labels
            - Query labels from database
        Operation: me
            - Query the authenticated member's data from database
        Operation: activity
            - Query activity logs from database with optional filters
```

# input

{{#if element.is_file}}

path: {{element.path}}
source:

```
{{element.content}}
```

{{else}}

path: {{element.path}}
sources:
{{#each element.children as |child|}}

- path: {{child.path}}
  source:
  ```
  {{child.content}}
  ```

{{/each}}

{{/if}}

give me only the output (in plain yaml format, don't use yaml code box syntax, only a parsable yaml result).
