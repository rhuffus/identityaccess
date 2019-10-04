-- Your SQL goes here
create table users
(
    id         serial primary key,
    uuid       uuid unique        not null,
    first_name varchar(30)        not null,
    last_name  varchar(40)        not null,
    username   varchar(20) unique not null,
    password   varchar(36)        not null,
    email      varchar(50) unique not null
--     created_at timestamp          not null,
--     updated_at timestamp

)
