CREATE EXTENSION IF NOT EXISTS md5hash;
CREATE EXTENSION IF NOT EXISTS moreint;
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
CREATE SCHEMA public;
SET search_path TO public;
CREATE OR REPLACE FUNCTION public.host_upsert(host_val text, tld_val text) RETURNS public.u64
    LANGUAGE plpgsql
    AS $$
DECLARE
    v_id u64;
    tld_id_val u32;
BEGIN
    -- 首先调用tld_upsert函数获取tld_id
    tld_id_val := tld_upsert(tld_val);
    -- 查询host表是否已经存在该记录
    SELECT id INTO v_id FROM host WHERE tld_id = tld_id_val AND val = host_val;
    -- 如果该记录不存在，则插入它
    IF v_id IS NULL THEN
        INSERT INTO host (tld_id, val) VALUES (tld_id_val, host_val) RETURNING id INTO v_id;
    END IF;
    RETURN v_id;
END;
$$;
CREATE OR REPLACE FUNCTION public.mail_upsert(prefix text, host_val text, tld_val text) RETURNS public.u64
    LANGUAGE plpgsql
    AS $$
DECLARE
    _host_id u64;
    mail_id u64;
BEGIN
    _host_id := host_upsert(host_val,tld_val);
    SELECT id INTO mail_id FROM mail WHERE host_id = _host_id AND val = prefix;
    IF mail_id IS NULL THEN
        INSERT INTO mail (host_id, val) VALUES (_host_id, prefix) RETURNING id INTO mail_id;
    END IF;
    RETURN mail_id;
END;
$$;
CREATE OR REPLACE FUNCTION public.tld_upsert(s text) RETURNS public.u64
    LANGUAGE plpgsql
    AS $$
DECLARE
    v_id u32;
BEGIN
    -- 首先查找给定的值
    SELECT id INTO v_id FROM tld WHERE val = s;
    -- 如果值不存在，则插入它
    IF v_id IS NULL THEN
        INSERT INTO tld (val) VALUES (s) RETURNING id INTO v_id;
    END IF;
    RETURN v_id;
END;
$$;
SET default_tablespace = '';
SET default_table_access_method = heap;
CREATE TABLE public.host (
    id public.u64 NOT NULL,
    tld_id public.u32 NOT NULL,
    val character varying(255) NOT NULL
);
CREATE SEQUENCE public.host_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.host_id_seq OWNED BY public.host.id;
CREATE TABLE public.mail (
    id bigint NOT NULL,
    val character varying(255) NOT NULL,
    host_id public.u64 NOT NULL
);
CREATE SEQUENCE public.mail_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.mail_id_seq OWNED BY public.mail.id;
CREATE TABLE public.tld (
    id public.u32 NOT NULL,
    val character varying(255) NOT NULL
);
CREATE SEQUENCE public.tld_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;
ALTER SEQUENCE public.tld_id_seq OWNED BY public.tld.id;
ALTER TABLE ONLY public.host ALTER COLUMN id SET DEFAULT nextval('public.host_id_seq'::regclass);
ALTER TABLE ONLY public.mail ALTER COLUMN id SET DEFAULT nextval('public.mail_id_seq'::regclass);
ALTER TABLE ONLY public.tld ALTER COLUMN id SET DEFAULT nextval('public.tld_id_seq'::regclass);
ALTER TABLE ONLY public.host
    ADD CONSTRAINT host_pkey PRIMARY KEY (id);
ALTER TABLE ONLY public.host
    ADD CONSTRAINT host_tld_id_val_key UNIQUE (tld_id, val);
ALTER TABLE ONLY public.mail
    ADD CONSTRAINT mail_host_id_val_key UNIQUE (host_id, val);
ALTER TABLE ONLY public.mail
    ADD CONSTRAINT mail_pkey PRIMARY KEY (id);
ALTER TABLE ONLY public.tld
    ADD CONSTRAINT tld_pkey PRIMARY KEY (id);
ALTER TABLE ONLY public.tld
    ADD CONSTRAINT tld_val_key UNIQUE (val);