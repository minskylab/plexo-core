table "labels" {
  schema = schema.public
  column "id" {
    null    = false
    type    = uuid
    default = sql("gen_random_uuid()")
  }
  column "created_at" {
    null    = false
    type    = timestamptz
    default = sql("now()")
  }
  column "updated_at" {
    null    = false
    type    = timestamptz
    default = sql("now()")
  }
  column "name" {
    null = false
    type = text
  }
  column "description" {
    null = true
    type = text
  }
  column "color" {
    null = true
    type = character_varying
  }
  primary_key {
    columns = [column.id]
  }
  index "labels_name_key" {
    unique  = true
    columns = [column.name]
  }
}
table "labels_by_tasks" {
  schema = schema.public
  column "label_id" {
    null = false
    type = uuid
  }
  column "task_id" {
    null = false
    type = uuid
  }
  primary_key {
    columns = [column.label_id, column.task_id]
  }
}
table "members" {
  schema = schema.public
  column "id" {
    null    = false
    type    = uuid
    default = sql("gen_random_uuid()")
  }
  column "created_at" {
    null    = false
    type    = timestamptz
    default = sql("now()")
  }
  column "updated_at" {
    null    = false
    type    = timestamptz
    default = sql("now()")
  }
  column "name" {
    null = false
    type = text
  }
  column "email" {
    null = false
    type = character_varying
  }
  column "password_hash" {
    null = true
    type = character_varying
  }
  column "github_id" {
    null = true
    type = character_varying
  }
  column "google_id" {
    null = true
    type = character_varying
  }
  column "photo_url" {
    null = true
    type = character_varying
  }
  column "role" {
    null = true
    type = character_varying
  }
  primary_key {
    columns = [column.id]
  }
  index "members_github_id_key" {
    unique  = true
    columns = [column.github_id]
  }
  index "members_google_id_key" {
    unique  = true
    columns = [column.google_id]
  }
}
table "members_by_projects" {
  schema = schema.public
  column "member_id" {
    null = false
    type = uuid
  }
  column "project_id" {
    null = false
    type = uuid
  }
  primary_key {
    columns = [column.member_id, column.project_id]
  }
}
table "members_by_teams" {
  schema = schema.public
  column "team_id" {
    null = false
    type = uuid
  }
  column "member_id" {
    null = false
    type = uuid
  }
  column "role" {
    null    = true
    type    = character_varying
    default = "Member"
  }
  primary_key {
    columns = [column.team_id, column.member_id]
  }
}
table "projects" {
  schema = schema.public
  column "id" {
    null    = false
    type    = uuid
    default = sql("gen_random_uuid()")
  }
  column "created_at" {
    null    = false
    type    = timestamptz
    default = sql("now()")
  }
  column "updated_at" {
    null    = false
    type    = timestamptz
    default = sql("now()")
  }
  column "name" {
    null = false
    type = text
  }
  column "prefix" {
    null = true
    type = character_varying
  }
  column "owner_id" {
    null = false
    type = uuid
  }
  column "description" {
    null = true
    type = text
  }
  column "lead_id" {
    null = true
    type = uuid
  }
  column "start_date" {
    null = true
    type = timestamp
  }
  column "due_date" {
    null = true
    type = timestamp
  }
  primary_key {
    columns = [column.id]
  }
  foreign_key "projects_owner_id_fkey" {
    columns     = [column.owner_id]
    ref_columns = [table.members.column.id]
    on_update   = CASCADE
    on_delete   = SET_NULL
  }
}
table "self" {
  schema = schema.public
  column "id" {
    null    = false
    type    = uuid
    default = sql("gen_random_uuid()")
  }
  column "created_at" {
    null    = false
    type    = timestamptz
    default = sql("now()")
  }
  column "updated_at" {
    null    = false
    type    = timestamptz
    default = sql("now()")
  }
  column "name" {
    null = false
    type = text
  }
  primary_key {
    columns = [column.id]
  }
}
table "tasks" {
  schema = schema.public
  column "id" {
    null    = false
    type    = uuid
    default = sql("gen_random_uuid()")
  }
  column "created_at" {
    null    = false
    type    = timestamptz
    default = sql("now()")
  }
  column "updated_at" {
    null    = false
    type    = timestamptz
    default = sql("now()")
  }
  column "title" {
    null = false
    type = text
  }
  column "description" {
    null = true
    type = text
  }
  column "owner_id" {
    null = false
    type = uuid
  }
  column "status" {
    null = true
    type = character_varying
  }
  column "priority" {
    null = true
    type = character_varying
  }
  column "due_date" {
    null = true
    type = timestamptz
  }
  column "project_id" {
    null = true
    type = uuid
  }
  column "lead_id" {
    null = true
    type = uuid
  }
  column "labels" {
    null = true
    type = jsonb
  }
  column "count" {
    null = false
    type = serial
  }
  primary_key {
    columns = [column.id]
  }
  foreign_key "tasks_owner_id_fkey" {
    columns     = [column.owner_id]
    ref_columns = [table.members.column.id]
    on_update   = CASCADE
    on_delete   = SET_NULL
  }
}
table "tasks_by_assignees" {
  schema = schema.public
  column "task_id" {
    null = false
    type = uuid
  }
  column "assignee_id" {
    null = false
    type = uuid
  }
  primary_key {
    columns = [column.task_id, column.assignee_id]
  }
  foreign_key "tasks_by_assignees_assignee_id_fkey" {
    columns     = [column.assignee_id]
    ref_columns = [table.members.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
  foreign_key "tasks_by_assignees_task_id_fkey" {
    columns     = [column.task_id]
    ref_columns = [table.tasks.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
}
table "tasks_by_projects" {
  schema = schema.public
  column "task_id" {
    null = false
    type = uuid
  }
  column "project_id" {
    null = false
    type = uuid
  }
  primary_key {
    columns = [column.task_id, column.project_id]
  }
  foreign_key "tasks_by_projects_project_fkey" {
    columns     = [column.project_id]
    ref_columns = [table.projects.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
  foreign_key "tasks_by_projects_task_fkey" {
    columns     = [column.task_id]
    ref_columns = [table.tasks.column.id]
    on_update   = CASCADE
    on_delete   = CASCADE
  }
}
table "teams" {
  schema = schema.public
  column "id" {
    null    = false
    type    = uuid
    default = sql("gen_random_uuid()")
  }
  column "created_at" {
    null    = false
    type    = timestamptz
    default = sql("now()")
  }
  column "updated_at" {
    null    = false
    type    = timestamptz
    default = sql("now()")
  }
  column "name" {
    null = false
    type = character_varying
  }
  column "owner_id" {
    null = false
    type = uuid
  }
  column "visibility" {
    null = true
    type = character_varying
  }
  column "prefix" {
    null = true
    type = text
  }
  primary_key {
    columns = [column.id]
  }
  index "teams_prefix_key" {
    unique  = true
    columns = [column.prefix]
  }
}
table "teams_by_projects" {
  schema = schema.public
  column "team_id" {
    null = false
    type = uuid
  }
  column "project_id" {
    null = false
    type = uuid
  }
  primary_key {
    columns = [column.team_id, column.project_id]
  }
}
schema "public" {
}
