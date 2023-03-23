# clerk
#### A multi-platform todo-list cli-application

## Usage
The use of this application is pretty simple. To see which options you have and which commands are available, just type the '-h' flag after a command.

```bash
$ clerk -h
# returns all the commands you could use

$ clerk
# returns your todolist 

```
## Example 
How to add a task with a subtask and flag it as doing 
```bash
$ clerk add "Main task"
# add the main task

$ clerk add "Sub task of the Main task" -s 0
# -s flag marks the input as a subcommand of index 0
# usage: clerk add <StringTask> -s <IndexOfMaintask>

$ clerk mark 0 0 -d
# marks the subtask 0 of maintask 0 as doing

$ clerk mark 0 -d
# marks the maintask as doing
```
Remove a main task or subtask
```bash
$ clerk mark 0 0 -r
```

## Config
These are the default locations for different operating systems
```
Linux:   /home/alice/.config/clerk
Windows: C:\Users\Alice\AppData\Roaming\meloencoding\clerk
macOS:   /Users/Alice/Library/Application Support/dev.meloencoding.clerk
```

```toml
[settings]
local = false # use local_location
page_size = 5 # set the page size when you run 'clerk'

[locations]
config_dir = '' # gets set when you first run the program. you could change it to a custom location 
local_location = '' # gets set when you first run the program. you could change it to a custom location 
remote_location = "" # you need to set this your self to the location of your compatible api

# if you use some sort of validation in your api, you could set these to check validation in your api
[api]
remote_key = "" 
app_id = ""
app_key = ""
```

## External use
If you want to use the external function you need to have a compatible API. In the source there is a file called 'clerk.ts'. If you 
have a expressjs server you could just follow the instructions in the file. If not, you just need two endpoints, '/show' and 
'/set'. Both are self explaining in function. 

After you've succesfully started your api you should now add it to the 'remote_location' option in the config file. Check the 'Config' 
Paragraph for the location of your OS

For questions, please open an issue on this project.
