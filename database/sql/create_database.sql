
CREATE USER 'exhaustMeServer'@'localhost' IDENTIFIED BY 'pwdExhaustMeServer';
GRANT ALL PRIVILEGES ON EXHAUST_ME TO 'exhaustMeServer'@'localhost';
USE EXHAUST_ME;

CREATE TABLE USERS
(
    ID INT NOT NULL AUTO_INCREMENT,
    NAME TEXT NOT NULL,
    PRIMARY KEY (ID)
);