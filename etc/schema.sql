drop table if exists tasks;
drop table if exists lists;

create table lists (
    id serial primary key,
    title varchar(150) not null
);

create table tasks (
    id serial primary key,
    title varchar(150) not null,
    is_done boolean not null default false,
    list_id integer not null,

    foreign key (list_id) references lists(id)
);

insert into lists (title) values
    ('test-list'),
    ('extra list'),
    ('one more list');

insert into tasks (title, is_done, list_id) values
    ('eat', true, 1),
    ('sleep', false, 1),
    ('repeat', false, 1),
    ('just do it', false, 2),
    ('did it again', true, 3);
