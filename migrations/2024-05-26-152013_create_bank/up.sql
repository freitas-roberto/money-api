CREATE TABLE banks (
   id            SERIAL       NOT NULL,
   code          TEXT         NOT NULL,
   name          TEXT         NOT NULL,
   created_at    TIMESTAMP    DEFAULT (timezone('utc', now())),
   updated_at    TIMESTAMP,
   CONSTRAINT banks_pk PRIMARY KEY(id),
   CONSTRAINT banks_uk01 UNIQUE(code),
   CONSTRAINT banks_uk02 UNIQUE(name)
);

CREATE INDEX banks_ix01 on banks(code);
CREATE INDEX banks_ix02 on banks(name);