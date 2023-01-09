use crate::sdk::member::{Member, MemberRole};

pub enum NewMemberPayloadAuthKind {
    Github,
    Google,
}

pub struct NewMemberPayload {
    pub auth_kind: NewMemberPayloadAuthKind,
    pub auth_id: String,
    pub email: String,

    pub name: String,

    pub role: Option<MemberRole>,
}

impl NewMemberPayload {
    pub fn new(
        auth_kind: NewMemberPayloadAuthKind,
        auth_id: String,
        email: String,
        name: String,
    ) -> Self {
        Self {
            auth_kind,
            auth_id,
            email,
            name,
            role: None,
        }
    }

    pub fn name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }

    pub fn role(&mut self, role: MemberRole) -> &mut Self {
        self.role = Some(role);
        self
    }
}

pub struct MembersFilter {
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<MemberRole>,
    pub github_id: Option<String>,
    pub google_id: Option<String>,
}

impl MembersFilter {
    pub fn new() -> Self {
        Self {
            name: None,
            email: None,
            role: None,
            github_id: None,
            google_id: None,
        }
    }

    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn set_email(&mut self, email: String) -> &mut Self {
        self.email = Some(email);
        self
    }

    pub fn set_role(&mut self, role: MemberRole) -> &mut Self {
        self.role = Some(role);
        self
    }

    pub fn set_github_id(&mut self, github_id: String) -> &mut Self {
        self.github_id = Some(github_id);
        self
    }

    pub fn set_google_id(&mut self, google_id: String) -> &mut Self {
        self.google_id = Some(google_id);
        self
    }
}
