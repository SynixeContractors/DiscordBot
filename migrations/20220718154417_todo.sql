-- Add migration script here
CREATE TABLE boards (
    name varchar(255) PRIMARY KEY,
    description varchar(255) NOT NULL
);

CREATE TYPE status as ENUM ('todo', 'in-progress', 'done');

CREATE TABLE task (
    id integer PRIMARY KEY,
    board varchar(255) NOT NULL,
    description varchar(255) NOT NULL,
    status status NOT NULL,
    assignee varchar(255) NOT NULL,
    FOREIGN KEY (board) REFERENCES boards(name)
);

CREATE TYPE actions as ENUM ('assign', 'unassign', 'done', 'undone');

CREATE TABLE task_action (
    id integer PRIMARY KEY,
    task_id integer NOT NULL,
    action varchar(255) NOT NULL,
    actor varchar(255) NOT NULL,
    FOREIGN KEY (task_id) REFERENCES task(id)
);
