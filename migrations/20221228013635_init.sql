--
-- PostgreSQL database dump
--

-- Dumped from database version 15.2
-- Dumped by pg_dump version 15.2

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: public; Type: SCHEMA; Schema: -; Owner: -
--

CREATE SCHEMA IF NOT EXISTS public;


--
-- Name: SCHEMA public; Type: COMMENT; Schema: -; Owner: -
--

-- COMMENT ON SCHEMA public IS 'standard public schema';


--
-- Name: set_current_timestamp_updated_at(); Type: FUNCTION; Schema: public; Owner: -
--

CREATE FUNCTION public.set_current_timestamp_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
DECLARE
  _new record;
BEGIN
  _new := NEW;
  _new."updated_at" = NOW();
  RETURN _new;
END;
$$;


SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: labels; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.labels (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    name text NOT NULL,
    description text,
    color character varying
);


--
-- Name: labels_by_tasks; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.labels_by_tasks (
    label_id uuid NOT NULL,
    task_id uuid NOT NULL
);


--
-- Name: members; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.members (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    name text NOT NULL,
    email character varying NOT NULL,
    password_hash character varying,
    github_id character varying,
    google_id character varying,
    photo_url character varying,
    role character varying
);


--
-- Name: members_by_projects; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.members_by_projects (
    member_id uuid NOT NULL,
    project_id uuid NOT NULL
);


--
-- Name: members_by_teams; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.members_by_teams (
    team_id uuid NOT NULL,
    member_id uuid NOT NULL,
    role character varying DEFAULT 'Member'::character varying
);


--
-- Name: projects; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.projects (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    name text NOT NULL,
    prefix character varying,
    owner_id uuid NOT NULL,
    description text,
    lead_id uuid,
    start_date timestamp with time zone,
    due_date timestamp with time zone
);


--
-- Name: self; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.self (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    name text NOT NULL
);


--
-- Name: tasks; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.tasks (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    title text NOT NULL,
    description text,
    owner_id uuid NOT NULL,
    status character varying,
    priority character varying,
    due_date timestamp with time zone,
    project_id uuid,
    lead_id uuid,
    labels jsonb,
    count integer NOT NULL,
    parent_id uuid
);


--
-- Name: tasks_by_assignees; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.tasks_by_assignees (
    task_id uuid NOT NULL,
    assignee_id uuid NOT NULL
);


--
-- Name: tasks_by_projects; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.tasks_by_projects (
    task_id uuid NOT NULL,
    project_id uuid NOT NULL
);


--
-- Name: tasks_count_seq; Type: SEQUENCE; Schema: public; Owner: -
--

CREATE SEQUENCE public.tasks_count_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


--
-- Name: tasks_count_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: -
--

ALTER SEQUENCE public.tasks_count_seq OWNED BY public.tasks.count;


--
-- Name: teams; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.teams (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    name character varying NOT NULL,
    owner_id uuid NOT NULL,
    visibility character varying,
    prefix text
);


--
-- Name: teams_by_projects; Type: TABLE; Schema: public; Owner: -
--

CREATE TABLE public.teams_by_projects (
    team_id uuid NOT NULL,
    project_id uuid NOT NULL
);


--
-- Name: tasks count; Type: DEFAULT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.tasks ALTER COLUMN count SET DEFAULT nextval('public.tasks_count_seq'::regclass);


--
-- Name: labels_by_tasks labels_by_tasks_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.labels_by_tasks
    ADD CONSTRAINT labels_by_tasks_pkey PRIMARY KEY (label_id, task_id);


--
-- Name: labels labels_name_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.labels
    ADD CONSTRAINT labels_name_key UNIQUE (name);


--
-- Name: labels labels_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.labels
    ADD CONSTRAINT labels_pkey PRIMARY KEY (id);


--
-- Name: members_by_projects members_by_projects_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.members_by_projects
    ADD CONSTRAINT members_by_projects_pkey PRIMARY KEY (member_id, project_id);


--
-- Name: members_by_teams members_by_teams_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.members_by_teams
    ADD CONSTRAINT members_by_teams_pkey PRIMARY KEY (team_id, member_id);


--
-- Name: members members_github_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.members
    ADD CONSTRAINT members_github_id_key UNIQUE (github_id);


--
-- Name: members members_google_id_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.members
    ADD CONSTRAINT members_google_id_key UNIQUE (google_id);


--
-- Name: members members_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.members
    ADD CONSTRAINT members_pkey PRIMARY KEY (id);


--
-- Name: projects projects_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_pkey PRIMARY KEY (id);


--
-- Name: self self_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.self
    ADD CONSTRAINT self_pkey PRIMARY KEY (id);


--
-- Name: tasks_by_assignees tasks_by_assignees_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.tasks_by_assignees
    ADD CONSTRAINT tasks_by_assignees_pkey PRIMARY KEY (task_id, assignee_id);


--
-- Name: tasks_by_projects tasks_by_projects_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.tasks_by_projects
    ADD CONSTRAINT tasks_by_projects_pkey PRIMARY KEY (task_id, project_id);


--
-- Name: tasks tasks_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.tasks
    ADD CONSTRAINT tasks_pkey PRIMARY KEY (id);


--
-- Name: teams_by_projects teams_by_projects_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.teams_by_projects
    ADD CONSTRAINT teams_by_projects_pkey PRIMARY KEY (team_id, project_id);


--
-- Name: teams teams_pkey; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.teams
    ADD CONSTRAINT teams_pkey PRIMARY KEY (id);


--
-- Name: teams teams_prefix_key; Type: CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.teams
    ADD CONSTRAINT teams_prefix_key UNIQUE (prefix);


--
-- Name: labels set_public_labels_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_public_labels_updated_at BEFORE UPDATE ON public.labels FOR EACH ROW EXECUTE FUNCTION public.set_current_timestamp_updated_at();


--
-- Name: TRIGGER set_public_labels_updated_at ON labels; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TRIGGER set_public_labels_updated_at ON public.labels IS 'trigger to set value of column "updated_at" to current timestamp on row update';


--
-- Name: members set_public_members_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_public_members_updated_at BEFORE UPDATE ON public.members FOR EACH ROW EXECUTE FUNCTION public.set_current_timestamp_updated_at();


--
-- Name: TRIGGER set_public_members_updated_at ON members; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TRIGGER set_public_members_updated_at ON public.members IS 'trigger to set value of column "updated_at" to current timestamp on row update';


--
-- Name: projects set_public_projects_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_public_projects_updated_at BEFORE UPDATE ON public.projects FOR EACH ROW EXECUTE FUNCTION public.set_current_timestamp_updated_at();


--
-- Name: TRIGGER set_public_projects_updated_at ON projects; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TRIGGER set_public_projects_updated_at ON public.projects IS 'trigger to set value of column "updated_at" to current timestamp on row update';


--
-- Name: self set_public_self_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_public_self_updated_at BEFORE UPDATE ON public.self FOR EACH ROW EXECUTE FUNCTION public.set_current_timestamp_updated_at();


--
-- Name: TRIGGER set_public_self_updated_at ON self; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TRIGGER set_public_self_updated_at ON public.self IS 'trigger to set value of column "updated_at" to current timestamp on row update';


--
-- Name: tasks set_public_tasks_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_public_tasks_updated_at BEFORE UPDATE ON public.tasks FOR EACH ROW EXECUTE FUNCTION public.set_current_timestamp_updated_at();


--
-- Name: TRIGGER set_public_tasks_updated_at ON tasks; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TRIGGER set_public_tasks_updated_at ON public.tasks IS 'trigger to set value of column "updated_at" to current timestamp on row update';


--
-- Name: teams set_public_teams_updated_at; Type: TRIGGER; Schema: public; Owner: -
--

CREATE TRIGGER set_public_teams_updated_at BEFORE UPDATE ON public.teams FOR EACH ROW EXECUTE FUNCTION public.set_current_timestamp_updated_at();


--
-- Name: TRIGGER set_public_teams_updated_at ON teams; Type: COMMENT; Schema: public; Owner: -
--

COMMENT ON TRIGGER set_public_teams_updated_at ON public.teams IS 'trigger to set value of column "updated_at" to current timestamp on row update';


--
-- Name: projects projects_owner_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES public.members(id) ON UPDATE CASCADE ON DELETE SET NULL;


--
-- Name: tasks_by_assignees tasks_by_assignees_assignee_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.tasks_by_assignees
    ADD CONSTRAINT tasks_by_assignees_assignee_id_fkey FOREIGN KEY (assignee_id) REFERENCES public.members(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: tasks_by_assignees tasks_by_assignees_task_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.tasks_by_assignees
    ADD CONSTRAINT tasks_by_assignees_task_id_fkey FOREIGN KEY (task_id) REFERENCES public.tasks(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: tasks_by_projects tasks_by_projects_project_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.tasks_by_projects
    ADD CONSTRAINT tasks_by_projects_project_fkey FOREIGN KEY (project_id) REFERENCES public.projects(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: tasks_by_projects tasks_by_projects_task_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.tasks_by_projects
    ADD CONSTRAINT tasks_by_projects_task_fkey FOREIGN KEY (task_id) REFERENCES public.tasks(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: tasks tasks_owner_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: -
--

ALTER TABLE ONLY public.tasks
    ADD CONSTRAINT tasks_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES public.members(id) ON UPDATE CASCADE ON DELETE SET NULL;


--
-- Name: SCHEMA public; Type: ACL; Schema: -; Owner: -
--

-- GRANT CREATE ON SCHEMA public TO web_access;


--
-- PostgreSQL database dump complete
--

