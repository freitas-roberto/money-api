CREATE TABLE users (
	id           SERIAL        NOT NULL,
	username     TEXT          NOT NULL, 
	password     TEXT          NOT NULL,
	is_admin     BOOLEAN       NOT NULL DEFAULT FALSE, 
	is_active    BOOLEAN       NOT NULL DEFAULT FALSE, 
    created_at   TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at   TIMESTAMP     NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT users_pk   PRIMARY KEY(id),
	CONSTRAINT users_uk01 UNIQUE(username)
);

CREATE INDEX users_ix01 on users(username);
