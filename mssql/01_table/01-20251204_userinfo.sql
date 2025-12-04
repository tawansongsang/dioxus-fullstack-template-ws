USE Template;

IF OBJECT_ID(N'dbo.UserInfo', N'U') IS NULL
CREATE TABLE dbo.UserInfo (
    UserInfoId int IDENTITY NOT NULL PRIMARY KEY CLUSTERED,
    Uuid uniqueidentifier NOT NULL UNIQUE NONCLUSTERED,
    Email nvarchar(50),
    PersonId char(13) NOT NULL UNIQUE NONCLUSTERED CHECK (LEN(PersonId) = 13 AND ISNUMERIC(PersonId) = 1),
    FirstNameTh nvarchar(50) NOT NULL,
    MiddleNameTh nvarchar(50),
    LastNameTh nvarchar(50),
    FirstNameEn nvarchar(50),
    MiddleNameEn nvarchar(50),
    LastNameEn nvarchar(50),
    Sex char(1) CHECK (Sex = 'm' OR Sex = 'f'),
    Nationality smallint, -- NOT NULL
    CreateOn datetime2 NOT NULL,
    CreateBy char(8) NOT NULL,
    UpdateOn datetime2,
    UpdateBy int FOREIGN KEY REFERENCES dbo.UserInfo(UserInfoId),
    Active char(1) NOT NULL CHECK (Active = 'Y' OR Active = 'N'),
    Deleted char(1) NOT NULL CHECK (Deleted = 'Y' OR Deleted = 'N'),
    DeleteOn datetime2,
    DeleteBy int FOREIGN KEY REFERENCES dbo.UserInfo(UserInfoId)
);
