# Example of web application with actix

This is a demo project to test actix framework functionalities. 

For testing the application using MongoDB, follow the following steps:
- Run `docker compose up -d` to start the MongoDB container.
- Create a database called `...`.
- Create a collection called `...`.
- Use [MongoDB Compass](https://www.mongodb.com/products/compass) to view the database content.
Note: MongoDB Compass is a tool that can be used to interact with MongoDB databases and inspect their content.

```shell
cargo watch -x run
```

Commands for building the app’s container image and starting the app container:
```shell
docker build -t server .
docker run -dp 8000:8000 server  
```