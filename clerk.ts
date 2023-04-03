// A basic expressjs endpoint. You need one endpoint that is able to handle the command "/show" and "/set"
// If you have a expressjs server you could add this by using 'app.use("/clerk", clerk);'. 
// If you decide to use this endpoint you should also have a file called 'data.json' in the exact 
// same location as this file.

// If you care about security and don't want people to use your api endpoint, you should at least
// set one of the security variables. don't forget to add these to your "config.toml"

// If you have any questions, please open an issue in github.

import * as express from "express";
import * as fs from "fs";
import path from "path";

const router = express.Router();

// you could just type a random string like "AbCd3fGhIJKlmn0p". this string will be used to validate your request.
const CLIENT_KEY = "AbCd3fGhIJKlmn0p"; 
// used mainly if you have some sort of api gateway and need to be able to identify what app is being called. but you could also use it 
// to add more depth to the security layer of this program.
const APP_ID = ""; 
const APP_KEY = ""; 

function show() {
    const todoItems: TodoItem[] = JSON.parse(fs.readFileSync(path.join(__dirname, "../../data/clerk.data.json")).toString());

    return { valid: true, data: todoItems };
}

function set(data: any) {
    fs.writeFileSync(path.join(__dirname, "../../data/clerk.data.json"), JSON.stringify(data, null, 2));

    return { valid: true };
}

router.post("/", (req, res) => {
    const { endpoint, data, clientKey, appId, appKey } = req.body;

    if (clientKey != CLIENT_KEY || appId != APP_ID || appKey != APP_KEY)
        return res.send({ valid: false });

    switch (endpoint) {
    case "/show":
        return res.send(show());
    case "/set":
        return res.send(set(data.list));
    }
});

export default router;

interface TodoItem {
    data: Todo[],
    title: string,
    state: TaskState,
    github_link: string,
}

enum TaskState {
    Pending = 0,
    Doing = 1,
    Completed = 2
}

interface Todo {
    data: string,
    state: TaskState
}