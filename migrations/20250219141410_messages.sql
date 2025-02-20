begin;
    create table messages (
        id serial primary key,
        conversation_id integer not null references conversations(id),
        model text,
        role text not null,
        body text not null,
        created_at timestamp not null default now(),
        updated_at timestamp not null default now()
    );

    create index on messages(conversation_id);
    create trigger set_updated_at before update on messages for each row execute function set_updated_at();
commit;
