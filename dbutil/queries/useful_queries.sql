-- Some useful queries for testing.

-- Summary of book and author information.
select b.id
     , b.title
     , group_concat( a.last_name order by a.last_name separator ", ") as authors
--      , a.first_name
     , b.isbn
     , b.publisher
  from book b
  join author a
  join author_book ab
 where ab.book_id = b.id and ab.author_id = a.id
 group by b.id
 order by b.title asc
;
-- TODO: Add formatted first name to authors list.

-- Count book and author rows.

select count(distinct b.id) -- # book rows
     , count(distinct a.id) -- # author rows
  from `book` b
  join `author` a
;
