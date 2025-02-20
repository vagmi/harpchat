-- Add migration script here
begin;
    create or replace function notify_message_on_create() returns trigger as $$
    begin
        perform pg_notify('messages:' || NEW.conversation_id, row_to_json(NEW)::text);
        return new;
    end;
    $$ language plpgsql;

    create trigger notify_message_on_create after insert on messages for each row execute function notify_message_on_create();
commit;
