create table devs (
    id serial primary key,
    name varchar,
    github varchar,
    bio text,
    avatar_url varchar,
    techs text[]
);