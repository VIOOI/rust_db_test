-- Your SQL goes here
CREATE TABLE public."user"
(
    id serial NOT NULL,
    login character varying(100) NOT NULL,
    email character varying(200) NOT NULL,
    password character varying(200) NOT NULL,
    first_name character varying(250),
    last_name character varying(250),
    PRIMARY KEY (id)
);

ALTER TABLE IF EXISTS public."user"
    OWNER to postgres;
