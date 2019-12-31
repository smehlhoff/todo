List incomplete todos
    
    curl 127.0.0.1:8080/todos -X GET

Create new todo

    curl 127.0.0.1:8080/todos -d "@data.json" -H "Content-Type: application/json" -X POST

List todo by id

    curl 127.0.0.1:8080/todos/1 -X GET

Update todo by id

    curl 127.0.0.1:8080/todos/1 -d "@data2.json" -H "Content-Type: application/json" -X PUT

Delete todo by id 

    curl 127.0.0.1:8080/todos/1 -X DELETE
