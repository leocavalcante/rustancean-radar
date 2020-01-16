create table devs
(
    id         serial primary key,
    name       varchar not null,
    github     varchar not null,
    bio        text    not null,
    avatar_url varchar not null,
    techs      text[]  not null,
    lat        float4  not null,
    lng        float4  not null
);