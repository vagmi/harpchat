begin;
    -- set_updated_at trigger function
    create or replace function set_updated_at() returns trigger as $$
    begin
        new.updated_at = now();
        return new;
    end;
    $$ language plpgsql;

    create table conversations (
        id serial primary key,
        title text not null default 'New Conversation',
        created_at timestamp not null default now(),
        updated_at timestamp not null default now()
    );


    create trigger set_updated_at before update on conversations for each row execute function set_updated_at();
commit;
