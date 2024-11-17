-- Setup our schema for the `collections` database.

drop table if exists `book`;
drop table if exists `author`;
drop table if exists `author_book`;

-- Create tables.

create table if not exists `book` (
       id int not null auto_increment
     , title text not null
     -- e.g. 978-0128230350
     , isbn varchar(14)
     , year smallint
     --
     , created datetime not null default current_timestamp
     , updated datetime not null default current_timestamp on update current_timestamp
     --
     , primary key (id)
) engine=innodb default charset=utf8mb4 collate=utf8mb4_general_ci
;

create table if not exists `author` (
       id int not null auto_increment
     , first_name text not null
     , middle_names text not null
     , last_name text not null
     --
     , created datetime not null default current_timestamp
     , updated datetime not null default current_timestamp on update current_timestamp
     --
     , primary key (id)
) engine=innodb default charset=utf8mb4 collate=utf8mb4_general_ci
;

create table if not exists `author_book` (
       id int not null auto_increment
     , author_id int not null
     , book_id int not null
     --
     , created datetime not null default current_timestamp
     , updated datetime not null default current_timestamp on update current_timestamp
     --
     , primary key (id)
     , unique key author_book (author_id, book_id)
) engine=innodb default charset=utf8mb4 collate=utf8mb4_general_ci
;