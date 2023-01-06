use async_graphql::{ComplexObject, Enum, SimpleObject};
use chrono::{DateTime, Utc};

use uuid::Uuid;

use super::{member::Member, project::Project};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum TaskStatus {
    None,
    Backlog,
    ToDo,
    InProgress,
    Done,
    Canceled,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum TaskPriority {
    None,
    Low,
    Medium,
    High,
    Urgent,
}

impl TaskStatus {
    pub fn from_optional_str(s: &Option<String>) -> Self {
        match s {
            Some(s) => Self::from_str(s.as_str()),
            None => Self::None,
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "None" => Self::None,
            "Backlog" => Self::Backlog,
            "ToDo" => Self::ToDo,
            "InProgress" => Self::InProgress,
            "Done" => Self::Done,
            "Canceled" => Self::Canceled,
            _ => Self::None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Backlog => "Backlog",
            Self::ToDo => "ToDo",
            Self::InProgress => "InProgress",
            Self::Done => "Done",
            Self::Canceled => "Canceled",
        }
    }
}

impl TaskPriority {
    pub fn from_optional_str(s: &Option<String>) -> Self {
        match s {
            Some(s) => Self::from_str(s.as_str()),
            None => Self::None,
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "None" => Self::None,
            "Low" => Self::Low,
            "Medium" => Self::Medium,
            "High" => Self::High,
            "Urgent" => Self::Urgent,
            _ => Self::None,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::Urgent => "Urgent",
        }
    }
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Task {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub title: String,
    pub description: Option<String>,

    pub status: TaskStatus,
    pub priority: TaskPriority,

    pub owner_id: Uuid,

    pub labels: Vec<String>,

    pub assignee_id: Option<Uuid>,
    pub project_id: Option<Uuid>,

    pub due_date: Option<DateTime<Utc>>,
}

#[ComplexObject]
impl Task {
    pub async fn owner(&self) -> Member {
        todo!()
    }

    pub async fn assignee(&self) -> Option<Member> {
        todo!()
    }

    pub async fn project(&self) -> Option<Project> {
        todo!()
    }
}

struct TaskBuilder;

impl TaskBuilder {
    // pub fn from_record(record: TaskRecord) -> Task {
    //     Task {
    //         id: record.id,
    //         created_at: record.created_at,
    //         updated_at: record.updated_at,
    //         title: record.title,
    //         description: record.description,
    //         status: record.status,
    //         priority: record.priority,
    //         owner_id: record.owner_id,
    //         labels: record.labels,
    //         assignee_id: record.assignee_id,
    //         project_id: record.project_id,
    //         due_date: record.due_date,
    //     }
    // }
}
