# Démarrer la BBD 

## Construire l'image

```sh
sudo docker build . -t exhaust_me_db:latest
```

## Lancer un nouveau container

```sh
sudo docker run --name exhaust_me_db -p 3306:3306 -it exhaust_me_db 
```