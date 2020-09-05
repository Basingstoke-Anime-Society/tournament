-- Your SQL goes here

create table anime (
    id integer unique not null auto_increment primary key,
    name varchar(255) not null,
    episodes integer,
    slot1 boolean not null default false,
    slot2 boolean not null default false,
    slot3 boolean not null default false
);

create table tournaments (
    id integer unique not null auto_increment primary key
);

create table tournament_anime (
    tournament integer not null,
    anime integer not null,
    primary key (tournament, anime)
);

create table decisions (
    tournament integer not null,
    left_anime integer not null,
    right_anime integer not null,
    pick boolean,
    primary key (tournament, left_anime, right_anime)
);