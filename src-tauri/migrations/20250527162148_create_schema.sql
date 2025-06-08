-- Add migration script here
CREATE TABLE Server (
	id INTEGER PRIMARY KEY NOT NULL,
	name TEXT,
	url TEXT,
	client_id TEXT
);

CREATE TABLE Message (
	id INTEGER PRIMARY KEY NOT NULL,
	topic TEXT,
	payload TEXT
);

CREATE TABLE Topic (
	id INTEGER PRIMARY KEY NOT NULL,
	FOREIGN KEY(fk_server_id) REFERENCES Server(id) NOT NULL,
	name TEXT,
	enabled BOOLEAN
);

CREATE TABLE Session (
	id INTEGER PRIMARY KEY NOT NULL,
	FOREIGN KEY(fk_selected_server_id) REFERENCES Server(id) NOT NULL,
	all_topics BOOLEAN,
	connected BOOLEAN
);
