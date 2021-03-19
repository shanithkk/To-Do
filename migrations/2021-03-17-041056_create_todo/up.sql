create table todo(
    id serial primary key,
    title varchar not null,
    checked boolean not null default false
);