-- Manually drops all data from database.

use collection
;

set FOREIGN_KEY_CHECKS = 0
;

truncate table `book`
;
truncate table `author`
;
truncate table `author_book`
;

set FOREIGN_KEY_CHECKS = 1
;
