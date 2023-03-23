// A basic expressjs endpoint. You'll need a '/show' and a '/set' endpoint that returns these types.
// If you have a expressjs server you could add this by using 'app.use("/clerk", clerk);'. 
// If you decide to use this endpoint you should also have a file called 'data.json' in the exact 
// same location as this file.

// If you have any questions, please open an issue in github.

import * as express from "express";
const router = express.Router();
import * as fs from "fs";

router.post('/show', (req, res) => {

    let todoItems: TodoItem[] = JSON.parse(fs.readFileSync("./data.json").toString());

    return res.json({
        valid: true,
        data: todoItems
    });
})

router.post('/set', (req, res) => { 
    let incomingList: TodoItem[] = req.body.list;

    fs.writeFileSync("./data.json", JSON.stringify(incomingList, null, 2));

    return res.json({
        valid: true
    });
})

export default router

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