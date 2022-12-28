--
-- PostgreSQL database dump
--

-- Dumped from database version 15.1
-- Dumped by pg_dump version 15.1 (Ubuntu 15.1-1.pgdg22.04+1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = off;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET escape_string_warning = off;
SET row_security = off;

--
-- Name: pgcrypto; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;


--
-- Name: EXTENSION pgcrypto; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION pgcrypto IS 'cryptographic functions';


--
-- Name: set_current_timestamp_updated_at(); Type: FUNCTION; Schema: public; Owner: bregydoc
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


ALTER FUNCTION public.set_current_timestamp_updated_at() OWNER TO bregydoc;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: members; Type: TABLE; Schema: public; Owner: bregydoc
--

CREATE TABLE public.members (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    name text NOT NULL,
    email character varying NOT NULL,
    password_hash character varying,
    github_id character varying,
    google_id character varying
);


ALTER TABLE public.members OWNER TO bregydoc;

--
-- Name: projects; Type: TABLE; Schema: public; Owner: bregydoc
--

CREATE TABLE public.projects (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    name text NOT NULL,
    prefix character varying NOT NULL,
    owner_id uuid
);


ALTER TABLE public.projects OWNER TO bregydoc;

--
-- Name: tasks; Type: TABLE; Schema: public; Owner: bregydoc
--

CREATE TABLE public.tasks (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    title text NOT NULL,
    description text,
    owner_id uuid
);


ALTER TABLE public.tasks OWNER TO bregydoc;

--
-- Name: tasks_by_assignees; Type: TABLE; Schema: public; Owner: bregydoc
--

CREATE TABLE public.tasks_by_assignees (
    task_id uuid NOT NULL,
    assignee_id uuid NOT NULL
);


ALTER TABLE public.tasks_by_assignees OWNER TO bregydoc;

--
-- Name: tasks_by_projects; Type: TABLE; Schema: public; Owner: bregydoc
--

CREATE TABLE public.tasks_by_projects (
    task_id uuid NOT NULL,
    project_id uuid NOT NULL
);


ALTER TABLE public.tasks_by_projects OWNER TO bregydoc;

--
-- Data for Name: members; Type: TABLE DATA; Schema: public; Owner: bregydoc
--

COPY public.members (id, created_at, updated_at, name, email, password_hash, github_id, google_id) FROM stdin;
bfa44f6f-200f-4a91-88b6-4524cfaf1685	2022-12-28 01:16:28.122701+00	2022-12-28 01:16:28.122701+00	Bregy Malpartida	bregy@minsky.cc	\N	\N	\N
\.


--
-- Data for Name: projects; Type: TABLE DATA; Schema: public; Owner: bregydoc
--

COPY public.projects (id, created_at, updated_at, name, prefix, owner_id) FROM stdin;
e82ca097-6878-4739-9e13-d69ef21dbd08	2022-12-27 23:58:02.625052+00	2022-12-28 01:26:05.477048+00	test	TEST	bfa44f6f-200f-4a91-88b6-4524cfaf1685
\.


--
-- Data for Name: tasks; Type: TABLE DATA; Schema: public; Owner: bregydoc
--

COPY public.tasks (id, created_at, updated_at, title, description, owner_id) FROM stdin;
2872033d-8781-4e79-a99d-746a60362b45	2022-12-28 01:16:39.784253+00	2022-12-28 01:16:39.784253+00	Issue 1	\N	bfa44f6f-200f-4a91-88b6-4524cfaf1685
\.


--
-- Data for Name: tasks_by_assignees; Type: TABLE DATA; Schema: public; Owner: bregydoc
--

COPY public.tasks_by_assignees (task_id, assignee_id) FROM stdin;
\.


--
-- Data for Name: tasks_by_projects; Type: TABLE DATA; Schema: public; Owner: bregydoc
--

COPY public.tasks_by_projects (task_id, project_id) FROM stdin;
2872033d-8781-4e79-a99d-746a60362b45	e82ca097-6878-4739-9e13-d69ef21dbd08
\.


--
-- Name: members members_pkey; Type: CONSTRAINT; Schema: public; Owner: bregydoc
--

ALTER TABLE ONLY public.members
    ADD CONSTRAINT members_pkey PRIMARY KEY (id);


--
-- Name: projects projects_pkey; Type: CONSTRAINT; Schema: public; Owner: bregydoc
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_pkey PRIMARY KEY (id);


--
-- Name: tasks_by_assignees tasks_by_assignees_pkey; Type: CONSTRAINT; Schema: public; Owner: bregydoc
--

ALTER TABLE ONLY public.tasks_by_assignees
    ADD CONSTRAINT tasks_by_assignees_pkey PRIMARY KEY (task_id, assignee_id);


--
-- Name: tasks_by_projects tasks_by_projects_pkey; Type: CONSTRAINT; Schema: public; Owner: bregydoc
--

ALTER TABLE ONLY public.tasks_by_projects
    ADD CONSTRAINT tasks_by_projects_pkey PRIMARY KEY (task_id, project_id);


--
-- Name: tasks tasks_pkey; Type: CONSTRAINT; Schema: public; Owner: bregydoc
--

ALTER TABLE ONLY public.tasks
    ADD CONSTRAINT tasks_pkey PRIMARY KEY (id);


--
-- Name: members set_public_members_updated_at; Type: TRIGGER; Schema: public; Owner: bregydoc
--

CREATE TRIGGER set_public_members_updated_at BEFORE UPDATE ON public.members FOR EACH ROW EXECUTE FUNCTION public.set_current_timestamp_updated_at();


--
-- Name: TRIGGER set_public_members_updated_at ON members; Type: COMMENT; Schema: public; Owner: bregydoc
--

COMMENT ON TRIGGER set_public_members_updated_at ON public.members IS 'trigger to set value of column "updated_at" to current timestamp on row update';


--
-- Name: projects set_public_projects_updated_at; Type: TRIGGER; Schema: public; Owner: bregydoc
--

CREATE TRIGGER set_public_projects_updated_at BEFORE UPDATE ON public.projects FOR EACH ROW EXECUTE FUNCTION public.set_current_timestamp_updated_at();


--
-- Name: TRIGGER set_public_projects_updated_at ON projects; Type: COMMENT; Schema: public; Owner: bregydoc
--

COMMENT ON TRIGGER set_public_projects_updated_at ON public.projects IS 'trigger to set value of column "updated_at" to current timestamp on row update';


--
-- Name: tasks set_public_tasks_updated_at; Type: TRIGGER; Schema: public; Owner: bregydoc
--

CREATE TRIGGER set_public_tasks_updated_at BEFORE UPDATE ON public.tasks FOR EACH ROW EXECUTE FUNCTION public.set_current_timestamp_updated_at();


--
-- Name: TRIGGER set_public_tasks_updated_at ON tasks; Type: COMMENT; Schema: public; Owner: bregydoc
--

COMMENT ON TRIGGER set_public_tasks_updated_at ON public.tasks IS 'trigger to set value of column "updated_at" to current timestamp on row update';


--
-- Name: projects projects_owner_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bregydoc
--

ALTER TABLE ONLY public.projects
    ADD CONSTRAINT projects_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES public.members(id) ON UPDATE CASCADE ON DELETE SET NULL;


--
-- Name: tasks_by_assignees tasks_by_assignees_assignee_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bregydoc
--

ALTER TABLE ONLY public.tasks_by_assignees
    ADD CONSTRAINT tasks_by_assignees_assignee_id_fkey FOREIGN KEY (assignee_id) REFERENCES public.members(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: tasks_by_assignees tasks_by_assignees_task_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bregydoc
--

ALTER TABLE ONLY public.tasks_by_assignees
    ADD CONSTRAINT tasks_by_assignees_task_id_fkey FOREIGN KEY (task_id) REFERENCES public.tasks(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: tasks_by_projects tasks_by_projects_project_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bregydoc
--

ALTER TABLE ONLY public.tasks_by_projects
    ADD CONSTRAINT tasks_by_projects_project_fkey FOREIGN KEY (project_id) REFERENCES public.projects(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: tasks_by_projects tasks_by_projects_task_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bregydoc
--

ALTER TABLE ONLY public.tasks_by_projects
    ADD CONSTRAINT tasks_by_projects_task_fkey FOREIGN KEY (task_id) REFERENCES public.tasks(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: tasks tasks_owner_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: bregydoc
--

ALTER TABLE ONLY public.tasks
    ADD CONSTRAINT tasks_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES public.members(id) ON UPDATE CASCADE ON DELETE SET NULL;


--
-- Name: SCHEMA public; Type: ACL; Schema: -; Owner: pg_database_owner
--

GRANT CREATE ON SCHEMA public TO web_access;


--
-- PostgreSQL database dump complete
--

