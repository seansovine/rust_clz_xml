-- Set up our schema for the `collections` database.

-- Needs to be first, unless we disable FKs for delete.
drop table if exists `author_book`;

drop table if exists `book`;
drop table if exists `author`;

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
     , middle_names text
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

-- Create foreign keys.
-- Note that default for on delete / update is no action.

    alter table `author_book`
add foreign key `author_book_author_fk` (`author_id`)
     references `author`(`id`)
;

    alter table `author_book`
add foreign key `author_book_book_fk` (`book_id`)
     references `book`(`id`)
;
