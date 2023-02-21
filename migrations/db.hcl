table "event_invocation_logs" {
  schema = schema.hdb_catalog
  column "id" {
    null    = false
    type    = text
    default = sql("hdb_catalog.gen_hasura_uuid()")
  }
  column "trigger_name" {
    null = true
    type = text
  }
  column "event_id" {
    null = true
    type = text
  }
  column "status" {
    null = true
    type = integer
  }
  column "request" {
    null = true
    type = json
  }
  column "response" {
    null = true
    type = json
  }
  column "created_at" {
    null    = true
    type    = timestamp
    default = sql("now()")
  }
  primary_key {
    columns = [column.id]
  }
  index "event_invocation_logs_event_id_idx" {
    columns = [column.event_id]
  }
}
table "event_log" {
  schema = schema.hdb_catalog
  column "id" {
    null    = false
    type    = text
    default = sql("hdb_catalog.gen_hasura_uuid()")
  }
  column "schema_name" {
    null = false
    type = text
  }
  column "table_name" {
    null = false
    type = text
  }
  column "trigger_name" {
    null = false
    type = text
  }
  column "payload" {
    null = false
    type = jsonb
  }
  column "delivered" {
    null    = false
    type    = boolean
    default = false
  }
  column "error" {
    null    = false
    type    = boolean
    default = false
  }
  column "tries" {
    null    = false
    type    = integer
    default = 0
  }
  column "created_at" {
    null    = true
    type    = timestamp
    default = sql("now()")
  }
  column "locked" {
    null = true
    type = timestamptz
  }
  column "next_retry_at" {
    null = true
    type = timestamp
  }
  column "archived" {
    null    = false
    type    = boolean
    default = false
  }
  primary_key {
    columns = [column.id]
  }
  index "event_log_fetch_events" {
    columns = [column.locked, column.next_retry_at, column.created_at]
    where   = "((delivered = false) AND (error = false) AND (archived = false))"
  }
  index "event_log_trigger_name_idx" {
    columns = [column.trigger_name]
  }
}
table "hdb_event_log_cleanups" {
  schema = schema.hdb_catalog
  column "id" {
    null    = false
    type    = text
    default = sql("hdb_catalog.gen_hasura_uuid()")
  }
  column "trigger_name" {
    null = false
    type = text
  }
  column "scheduled_at" {
    null = false
    type = timestamp
  }
  column "deleted_event_logs" {
    null = true
    type = integer
  }
  column "deleted_event_invocation_logs" {
    null = true
    type = integer
  }
  column "status" {
    null = false
    type = text
  }
  primary_key {
    columns = [column.id]
  }
  index "hdb_event_log_cleanups_trigger_name_scheduled_at_key" {
    unique  = true
    columns = [column.trigger_name, column.scheduled_at]
  }
  check "hdb_event_log_cleanups_status_check" {
    expr = "(status = ANY (ARRAY['scheduled'::text, 'paused'::text, 'completed'::text, 'dead'::text]))"
  }
}
table "hdb_source_catalog_version" {
  schema = schema.hdb_catalog
  column "version" {
    null = false
    type = text
  }
  column "upgraded_on" {
    null = false
    type = timestamptz
  }
  index "hdb_source_catalog_version_one_row" {
    unique = true
    on {
      expr = "((version IS NOT NULL))"
    }
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
    null = false
    type = character_varying
  }
  column "owner_id" {
    null = true
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
  index "projects_prefix_key" {
    unique  = true
    columns = [column.prefix]
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
    null = true
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
schema "hdb_catalog" {
}
schema "public" {
}
