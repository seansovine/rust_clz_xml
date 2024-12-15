package data

type BookRecord struct {
	Title string

	Year      *int32
	Isbn      *string
	Publisher *string

	Authors []AuthorRecord
}

type AuthorRecord struct {
	FirstName *string
	LastName  *string
}
