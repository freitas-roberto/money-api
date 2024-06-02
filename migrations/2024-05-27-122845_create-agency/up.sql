CREATE TABLE agencies (
   id            SERIAL       NOT NULL,
   code          TEXT         NOT NULL,
   name          TEXT         NOT NULL,
   bank_id       INT          NOT NULL,
   created_at    TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
   updated_at    TIMESTAMP    NOT NULL DEFAULT CURRENT_TIMESTAMP,
   CONSTRAINT agencies_pk PRIMARY KEY(id),
   CONSTRAINT agencies_uk01 UNIQUE(bank_id, code),
   CONSTRAINT agencies_uk02 UNIQUE(bank_id, name)
);

CREATE INDEX agencies_ix01 on agencies(code);
CREATE INDEX agencies_ix02 on agencies(name);

ALTER TABLE agencies
ADD CONSTRAINT agencies_fk01
FOREIGN KEY (bank_id) REFERENCES banks(id);