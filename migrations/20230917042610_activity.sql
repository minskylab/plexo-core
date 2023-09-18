-- Add migration script here

CREATE TABLE public.activity (
    id uuid DEFAULT gen_random_uuid() NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    
    member_id uuid NOT NULL,
    resource_id uuid NOT NULL,

    operation text NOT NULL,
    resource_type text NOT NULL
);


ALTER TABLE ONLY public.activity
    ADD CONSTRAINT activity_pkey PRIMARY KEY (id);

ALTER TABLE ONLY public.activity
    ADD CONSTRAINT activity_member_id_fkey FOREIGN KEY (member_id) REFERENCES public.members(id) ON DELETE CASCADE;

CREATE INDEX activity_member_id_idx ON public.activity USING btree (member_id);

CREATE INDEX activity_resource_id_idx ON public.activity USING btree (resource_id);