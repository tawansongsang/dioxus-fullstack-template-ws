USE master;
GO
IF DB_ID (N'Template') IS NULL
CREATE DATABASE Template 
ON
(NAME = Template_data,
    FILENAME = '/var/opt/mssql/data/Template_data.mdf',
    SIZE = 8 MB,
    MAXSIZE = UNLIMITED,
    FILEGROWTH = 80%)
LOG ON
(NAME = Template_log,
    FILENAME = '/var/opt/mssql/data/Template_log.ldf',
    SIZE = 8 MB,
    MAXSIZE = UNLIMITED,
    FILEGROWTH = 80%)
COLLATE Thai_CI_AS;
GO