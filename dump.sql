--
-- PostgreSQL database dump
--

-- Dumped from database version 14.0
-- Dumped by pg_dump version 14.0

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: test_table; Type: TABLE; Schema: public; Owner: iamspazzy
--

CREATE TABLE public.test_table (
    id integer NOT NULL,
    data character varying(64)
);


ALTER TABLE public.test_table OWNER TO iamspazzy;

--
-- Name: test_table_id_seq; Type: SEQUENCE; Schema: public; Owner: iamspazzy
--

CREATE SEQUENCE public.test_table_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.test_table_id_seq OWNER TO iamspazzy;

--
-- Name: test_table_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: iamspazzy
--

ALTER SEQUENCE public.test_table_id_seq OWNED BY public.test_table.id;


--
-- Name: users; Type: TABLE; Schema: public; Owner: iamspazzy
--

CREATE TABLE public.users (
    id integer NOT NULL,
    email character varying(32),
    password character varying(128),
    salt character varying(64)
);


ALTER TABLE public.users OWNER TO iamspazzy;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: public; Owner: iamspazzy
--

CREATE SEQUENCE public.users_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.users_id_seq OWNER TO iamspazzy;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: iamspazzy
--

ALTER SEQUENCE public.users_id_seq OWNED BY public.users.id;


--
-- Name: test_table id; Type: DEFAULT; Schema: public; Owner: iamspazzy
--

ALTER TABLE ONLY public.test_table ALTER COLUMN id SET DEFAULT nextval('public.test_table_id_seq'::regclass);


--
-- Name: users id; Type: DEFAULT; Schema: public; Owner: iamspazzy
--

ALTER TABLE ONLY public.users ALTER COLUMN id SET DEFAULT nextval('public.users_id_seq'::regclass);


--
-- Data for Name: test_table; Type: TABLE DATA; Schema: public; Owner: iamspazzy
--

COPY public.test_table (id, data) FROM stdin;
1	hello
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: iamspazzy
--

COPY public.users (id, email, password, salt) FROM stdin;
507301	IamSpazzy@gmail.com	9C60A6A86B3364C4A21C055CEC89C610B72BB4C683E5ADAFD48DE032DE5991BB09BCAC3B8E3FB44C13459731557D3C261D08EEC90CB437F471CA433E9E125CF7	B99FABDA49044953A0C8E1F9D57FFC3B
\.


--
-- Name: test_table_id_seq; Type: SEQUENCE SET; Schema: public; Owner: iamspazzy
--

SELECT pg_catalog.setval('public.test_table_id_seq', 1, true);


--
-- Name: users_id_seq; Type: SEQUENCE SET; Schema: public; Owner: iamspazzy
--

SELECT pg_catalog.setval('public.users_id_seq', 507301, true);


--
-- Name: test_table test_table_pkey; Type: CONSTRAINT; Schema: public; Owner: iamspazzy
--

ALTER TABLE ONLY public.test_table
    ADD CONSTRAINT test_table_pkey PRIMARY KEY (id);


--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: iamspazzy
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (email);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: iamspazzy
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: users_email_idx; Type: INDEX; Schema: public; Owner: iamspazzy
--

CREATE INDEX users_email_idx ON public.users USING btree (email);


--
-- PostgreSQL database dump complete
--

