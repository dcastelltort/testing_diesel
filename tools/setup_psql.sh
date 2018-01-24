#!/bin/sh
docker run -p 5432:5432 --name local-postgres -e POSTGRES_PASSWORD=mysecretpassword -d postgres
export DATABASE_URL=postgres://postgres:mysecretpassword@localhost/testing_diesel

