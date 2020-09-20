
#Base mariadb image
FROM mariadb
#Environnement variables
ENV MYSQL_ROOT_PASSWORD=pwd 
ENV MYSQL_USER=exhaustMeServer
ENV MYSQL_PASSWORD=pwdExhaustMeServer 
ENV MYSQL_DATABASE=EXHAUST_ME

EXPOSE 3306

ADD sql/* /docker-entrypoint-initdb.d/



